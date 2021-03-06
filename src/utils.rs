use base_62::base62;
use na::{IsometryMatrix3, Matrix4, Point3, Vector2, Vector3, Vector4};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

pub fn face_towards_dir(
    pos: &Vector3<f32>,
    normalized_dir: &Vector3<f32>,
    normalized_up: &Vector3<f32>,
) -> Matrix4<f32> {
    let x_axis = normalized_dir;
    let y_axis = normalized_up.cross(&x_axis);
    let z_axis = x_axis.cross(&y_axis);

    Matrix4::new(
        x_axis.x, y_axis.x, z_axis.x, pos.x, //
        x_axis.y, y_axis.y, z_axis.y, pos.y, //
        x_axis.z, y_axis.z, z_axis.z, pos.z, //
        0.0, 0.0, 0.0, 1.0,
    )
}

const ID_CHARS: [char; 62] = [
    'A', 'Z', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'Q', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L',
    'M', 'W', 'X', 'C', 'V', 'B', 'N', 'a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'q', 's',
    'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'w', 'x', 'c', 'v', 'b', 'n', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];
const ID_SIZE: usize = 5;

trait IdBase {
    type Type;
}

pub type IdValue = u64;

#[derive(Serialize, Deserialize)]
pub struct Id<T> {
    pub value: IdValue,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: IdValue) -> Self {
        Id {
            value,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: typename::TypeName> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: typename::TypeName> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x: [u8; 8] = unsafe { std::mem::transmute(self.value.to_le()) };

        let name = format!("{:?}", T::type_name());

        let simple_name = name.split("::").last().unwrap();
        let simple_name = &simple_name[..simple_name.len() - 1];
        write!(
            f,
            "{}#{}",
            simple_name,
            base62::encode(&x) // format!("{:X}", self.value)
        )
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::new(self.value)
    }
}
impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Hash for Id<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.value.hash(state);
    }
}

impl<T> Eq for Id<T> {}

impl<T> IdBase for Id<T> {
    type Type = T;
}

pub fn rand_id<T>() -> Id<T> {
    Id::new(rand::prelude::random())
}

pub fn rand_id_unsafe() -> String {
    let mut rng = thread_rng();
    let mut s = String::with_capacity(ID_SIZE);
    for _ in 0..ID_SIZE {
        s.push(*ID_CHARS.choose(&mut rng).unwrap());
    }
    s
}

pub fn pop_set<T: Clone + Eq + std::hash::Hash>(set: &mut HashSet<T>) -> T {
    let elt = set.iter().next().cloned().unwrap();
    set.take(&elt).unwrap()
}

pub fn time<F, K>(f: F) -> std::time::Duration
where
    F: FnOnce() -> K,
{
    let start = std::time::Instant::now();
    f();
    start.elapsed()
}
use std::fs::{self, DirEntry};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum FileTree {
    Unknown,
    Node {
        path: PathBuf,
        children: Vec<FileTree>,
    },
    Leaf {
        path: PathBuf,
    },
}

impl FileTree {
    pub fn new(path: PathBuf) -> Self {
        if path.is_dir() {
            let mut nodes = Vec::new();
            for entry in fs::read_dir(path.clone()).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                nodes.push(Self::new(path));
            }
            FileTree::Node {
                path: path.to_owned(),
                children: nodes,
            }
        } else {
            FileTree::Leaf { path }
        }
    }
}

pub struct ImageRGBA8 {
    pub w: u32,
    pub h: u32,
    pub data: Vec<u8>,
}

impl ImageRGBA8 {
    pub fn open(path: &str) -> ImageRGBA8 {
        use byteorder::{BigEndian, ReadBytesExt};
        use std::fs::File;

        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default output transformation is `Transformations::EXPAND
        // | Transformations::STRIP_ALPHA`.
        let mut decoder = png::Decoder::new(File::open(path).unwrap());
        decoder.set_transformations(png::Transformations::IDENTITY);
        let (info, mut reader) = decoder.read_info().unwrap();

        // Display image metadata.
        log::debug!("info: {:?}", info.width);
        log::debug!("height: {:?}", info.height);
        log::debug!("bit depth: {:?}", info.bit_depth);
        log::debug!("buffer size: {:?}", info.buffer_size());

        // Allocate the output buffer.
        let mut buf = vec![0; info.buffer_size()];
        // Read the next frame. Currently this function should only called once.
        // The default options
        reader.next_frame(&mut buf).unwrap();
        ImageRGBA8 {
            w: info.width,
            h: info.height,
            data: buf,
        }
    }
}
