use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Metadata {
    npts_u32: u32,
    ndims_u32: u32,
}

#[link(name = "c_file_rw_utils", kind = "static")]
extern "C" {
    fn open_fp_to_read(filename: *const u8, fp: *mut *mut std::os::raw::c_void) -> i32;
    fn open_fp_to_rw(filename: *const u8, fp: *mut *mut std::os::raw::c_void) -> i32;
    fn drop_fp(fp: *mut std::os::raw::c_void) -> i32;
    fn read_metadata(fp: *mut std::os::raw::c_void, metadata: *mut Metadata) -> i32;
    fn get_vector_f32(
        fp: *mut std::os::raw::c_void,
        metadata: *const Metadata,
        idx: usize,
        vec: *mut f32,
    ) -> i32;
    fn get_vector_u8(
        fp: *mut std::os::raw::c_void,
        metadata: *const Metadata,
        idx: usize,
        vec: *mut u8,
    ) -> i32;
    fn set_vector_f32(
        fp: *mut std::os::raw::c_void,
        metadata: *const Metadata,
        idx: usize,
        vec: *const f32,
    ) -> i32;
    fn set_vector_u8(
        fp: *mut std::os::raw::c_void,
        metadata: *const Metadata,
        idx: usize,
        vec: *const u8,
    ) -> i32;
    fn create_empty_f32bin_of_size(filename: *const u8, metadata: *const Metadata) -> i32;
    fn create_empty_u8bin_of_size(filename: *const u8, metadata: *const Metadata) -> i32;
}

// Read f32 binary file using Rust file reader
fn rust_read_file<T: Sized + Clone + Default>(filename: String) -> io::Result<(Metadata, Vec<T>)> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut metadata: Metadata = Default::default();

    // Read the metadata
    reader.read_exact(unsafe {
        std::slice::from_raw_parts_mut(
            &mut metadata as *mut Metadata as *mut u8,
            std::mem::size_of::<Metadata>(),
        )
    })?;

    // Read the vectors
    let mut vec: Vec<T> =
        vec![T::default(); metadata.ndims_u32 as usize * metadata.npts_u32 as usize];
    reader.read_exact(unsafe {
        std::slice::from_raw_parts_mut(
            vec.as_mut_ptr() as *mut u8,
            std::mem::size_of::<T>() * vec.len(),
        )
    })?;
    Ok((metadata, vec))
}

fn convert_fbin_to_u8bin(fbin_filename: String, u8bin_filename: String) -> io::Result<()> {
    let (metadata, fbin_vec) = rust_read_file::<f32>(fbin_filename)?;
    let u8bin_vec: Vec<u8> = fbin_vec.iter().map(|&x| x as u8).collect();

    let mut file = File::create(u8bin_filename)?;
    file.write_all(unsafe {
        std::slice::from_raw_parts(
            &metadata as *const Metadata as *const u8,
            std::mem::size_of::<Metadata>(),
        )
    })?;
    file.write_all(&u8bin_vec)?;
    Ok(())
}

pub struct CVecProviderU8 {
    fp: *mut std::os::raw::c_void,
    metadata: Metadata,
}

impl CVecProviderU8 {
    // Send Some(metadata) to overwrite a file
    // Send None to read an existing file
    pub fn new(filename: String, metadata_option: Option<Metadata>) -> Self {
        let cfilename = CString::new(filename).unwrap();
        let mut fp: *mut std::os::raw::c_void = std::ptr::null_mut();
        if let Some(metadata) = metadata_option {
            let ret =
                unsafe { create_empty_u8bin_of_size(cfilename.as_bytes().as_ptr(), &metadata) };
            assert_eq!(ret, 0);
        }
        let ret = unsafe { open_fp_to_rw(cfilename.as_bytes().as_ptr(), &mut fp) };
        assert_eq!(ret, 0);
        let mut metadata: Metadata = Default::default();
        let ret = unsafe { read_metadata(fp, &mut metadata) };
        assert_eq!(ret, 0);
        Self { fp, metadata }
    }

    pub fn drop(&self) {
        let drop_ret = unsafe { drop_fp(self.fp) };
        assert_eq!(drop_ret, 0);
    }

    pub fn get_vector_u8(&self, idx: usize) -> Result<Vec<u8>, String> {
        let mut vec: Vec<u8> = vec![0u8; self.metadata.ndims_u32 as usize];
        println!("fp: {:?}", self.fp);
        println!("vec len: {}", vec.len());
        let ret = unsafe { get_vector_u8(self.fp, &self.metadata, idx, vec.as_mut_ptr()) };
        match ret {
            0 => Ok(vec),
            _ => Err(format!("Failed to get vector at index {}", idx)),
        }
    }

    pub fn set_vector_u8(&self, idx: usize, vec: &[u8]) -> Result<(), String> {
        let ret = unsafe { set_vector_u8(self.fp, &self.metadata, idx, vec.as_ptr()) };
        match ret {
            0 => Ok(()),
            _ => Err(format!("Failed to set vector at index {}", idx)),
        }
    }
}

static SIFT_10K_FBIN_FILENAME: &str = "sift_query.fbin";
static SIFT_10K_U8BIN_FILENAME: &str = "sift_query.u8bin";

static SIFT_10K_FIRST_VEC_F32: [f32; 128] = [
    1.0, 3.0, 11.0, 110.0, 62.0, 22.0, 4.0, 0.0, 43.0, 21.0, 22.0, 18.0, 6.0, 28.0, 64.0, 9.0,
    11.0, 1.0, 0.0, 0.0, 1.0, 40.0, 101.0, 21.0, 20.0, 2.0, 4.0, 2.0, 2.0, 9.0, 18.0, 35.0, 1.0,
    1.0, 7.0, 25.0, 108.0, 116.0, 63.0, 2.0, 0.0, 0.0, 11.0, 74.0, 40.0, 101.0, 116.0, 3.0, 33.0,
    1.0, 1.0, 11.0, 14.0, 18.0, 116.0, 116.0, 68.0, 12.0, 5.0, 4.0, 2.0, 2.0, 9.0, 102.0, 17.0,
    3.0, 10.0, 18.0, 8.0, 15.0, 67.0, 63.0, 15.0, 0.0, 14.0, 116.0, 80.0, 0.0, 2.0, 22.0, 96.0,
    37.0, 28.0, 88.0, 43.0, 1.0, 4.0, 18.0, 116.0, 51.0, 5.0, 11.0, 32.0, 14.0, 8.0, 23.0, 44.0,
    17.0, 12.0, 9.0, 0.0, 0.0, 19.0, 37.0, 85.0, 18.0, 16.0, 104.0, 22.0, 6.0, 2.0, 26.0, 12.0,
    58.0, 67.0, 82.0, 25.0, 12.0, 2.0, 2.0, 25.0, 18.0, 8.0, 2.0, 19.0, 42.0, 48.0, 11.0,
];

static SIFT_10K_LAST_VEC_F32: [f32; 128] = [
    23.0, 0.0, 0.0, 0.0, 1.0, 5.0, 43.0, 114.0, 3.0, 0.0, 0.0, 0.0, 14.0, 121.0, 120.0, 78.0, 81.0,
    4.0, 0.0, 0.0, 31.0, 126.0, 23.0, 18.0, 126.0, 12.0, 0.0, 0.0, 0.0, 1.0, 0.0, 10.0, 0.0, 0.0,
    0.0, 0.0, 8.0, 29.0, 96.0, 43.0, 0.0, 0.0, 0.0, 0.0, 1.0, 81.0, 126.0, 44.0, 126.0, 1.0, 0.0,
    0.0, 1.0, 45.0, 66.0, 96.0, 126.0, 0.0, 0.0, 0.0, 1.0, 16.0, 12.0, 63.0, 1.0, 2.0, 0.0, 0.0,
    11.0, 40.0, 26.0, 0.0, 5.0, 20.0, 28.0, 1.0, 0.0, 17.0, 36.0, 5.0, 126.0, 45.0, 10.0, 1.0, 0.0,
    2.0, 12.0, 29.0, 126.0, 6.0, 0.0, 0.0, 2.0, 110.0, 96.0, 46.0, 18.0, 13.0, 0.0, 0.0, 3.0, 5.0,
    1.0, 2.0, 29.0, 50.0, 30.0, 7.0, 8.0, 3.0, 0.0, 1.0, 55.0, 24.0, 14.0, 5.0, 9.0, 15.0, 8.0,
    10.0, 10.0, 1.0, 0.0, 0.0, 19.0, 79.0, 16.0, 4.0,
];

static SIFT_LEARN_FIRST_VEC_F32: [f32; 128] = [
    97.0, 18.0, 9.0, 9.0, 0.0, 0.0, 2.0, 36.0, 21.0, 16.0, 59.0, 106.0, 64.0, 0.0, 5.0, 27.0, 5.0,
    38.0, 126.0, 68.0, 19.0, 0.0, 0.0, 2.0, 6.0, 38.0, 26.0, 15.0, 4.0, 0.0, 0.0, 2.0, 61.0, 16.0,
    8.0, 16.0, 28.0, 19.0, 15.0, 50.0, 126.0, 95.0, 44.0, 6.0, 9.0, 12.0, 7.0, 59.0, 5.0, 33.0,
    88.0, 60.0, 101.0, 95.0, 5.0, 1.0, 69.0, 49.0, 11.0, 20.0, 61.0, 25.0, 0.0, 2.0, 46.0, 18.0,
    0.0, 0.0, 24.0, 38.0, 15.0, 68.0, 116.0, 18.0, 0.0, 0.0, 8.0, 63.0, 57.0, 126.0, 7.0, 1.0, 0.0,
    0.0, 34.0, 126.0, 90.0, 26.0, 109.0, 8.0, 0.0, 3.0, 15.0, 39.0, 20.0, 72.0, 4.0, 38.0, 12.0,
    3.0, 7.0, 9.0, 1.0, 14.0, 2.0, 4.0, 4.0, 11.0, 14.0, 26.0, 57.0, 50.0, 11.0, 8.0, 9.0, 3.0,
    1.0, 60.0, 56.0, 6.0, 111.0, 4.0, 1.0, 1.0, 0.0, 2.0, 4.0, 26.0,
];

static SIFT_LEARN_LAST_VEC_F32: [f32; 128] = [
    49.0, 10.0, 16.0, 37.0, 5.0, 0.0, 0.0, 16.0, 113.0, 67.0, 17.0, 5.0, 2.0, 4.0, 17.0, 29.0,
    66.0, 12.0, 5.0, 14.0, 5.0, 7.0, 32.0, 54.0, 6.0, 0.0, 3.0, 31.0, 32.0, 33.0, 9.0, 1.0, 69.0,
    41.0, 5.0, 46.0, 28.0, 11.0, 28.0, 21.0, 113.0, 113.0, 35.0, 12.0, 3.0, 3.0, 4.0, 31.0, 57.0,
    70.0, 55.0, 7.0, 48.0, 29.0, 2.0, 13.0, 6.0, 0.0, 0.0, 0.0, 84.0, 113.0, 10.0, 4.0, 66.0, 73.0,
    30.0, 64.0, 16.0, 3.0, 11.0, 36.0, 84.0, 74.0, 10.0, 3.0, 6.0, 35.0, 81.0, 84.0, 17.0, 6.0,
    8.0, 1.0, 64.0, 113.0, 55.0, 95.0, 0.0, 0.0, 0.0, 1.0, 113.0, 113.0, 42.0, 2.0, 20.0, 3.0, 5.0,
    3.0, 7.0, 7.0, 11.0, 102.0, 21.0, 6.0, 1.0, 5.0, 33.0, 64.0, 43.0, 68.0, 21.0, 7.0, 3.0, 29.0,
    69.0, 67.0, 26.0, 68.0, 0.0, 0.0, 0.0, 21.0, 67.0, 66.0, 27.0, 1.0,
];

#[test]
fn test_create_file() {
    let sift_10K_path = format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        SIFT_10K_FBIN_FILENAME
    );
    let filename = CString::new(sift_10K_path).unwrap();

    let mut fp: *mut std::os::raw::c_void = std::ptr::null_mut();
    let ret = unsafe { open_fp_to_read(filename.as_bytes().as_ptr(), &mut fp) };
    assert_eq!(ret, 0);

    let drop_ret = unsafe { drop_fp(fp) };
    assert_eq!(drop_ret, 0);
}

#[test]
fn test_get_vector_f32() {
    let sift_10K_path = format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        SIFT_10K_FBIN_FILENAME
    );
    let filename = CString::new(sift_10K_path).unwrap();

    let mut fp: *mut std::os::raw::c_void = std::ptr::null_mut();
    let ret = unsafe { open_fp_to_read(filename.as_bytes().as_ptr(), &mut fp) };
    assert_eq!(ret, 0);

    // Check the metadata
    let mut metadata: Metadata = Default::default();
    let ret = unsafe { read_metadata(fp, &mut metadata) };
    assert_eq!(ret, 0);
    assert_eq!(metadata.npts_u32, 10_000);
    assert_eq!(metadata.ndims_u32, 128);

    // Check the first vector
    let mut vec: Vec<f32> = vec![0.0; metadata.ndims_u32 as usize];
    let ret = unsafe { get_vector_f32(fp, &metadata, 0, vec.as_mut_ptr()) };
    assert_eq!(ret, 0);
    assert_eq!(vec, SIFT_10K_FIRST_VEC_F32);

    // Check the last vector
    let last_vec_ret = unsafe { get_vector_f32(fp, &metadata, 9_999, vec.as_mut_ptr()) };
    assert_eq!(last_vec_ret, 0);
    assert_eq!(vec, SIFT_10K_LAST_VEC_F32);

    // Check drop file ptr
    let drop_ret = unsafe { drop_fp(fp) };
    assert_eq!(drop_ret, 0);
}

#[test]
fn test_rust_read_f32_file() {
    let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let sift_10K_path = format!("{}/{}", cargo_manifest_dir, SIFT_10K_FBIN_FILENAME);

    let (metadata, rust_vec) = rust_read_file::<f32>(sift_10K_path).unwrap();
    assert_eq!(metadata.npts_u32, 10_000);
    assert_eq!(metadata.ndims_u32, 128);
    assert_eq!(
        rust_vec.len(),
        metadata.npts_u32 as usize * metadata.ndims_u32 as usize
    );
    assert_eq!(rust_vec[0..128], SIFT_10K_FIRST_VEC_F32);
    assert_eq!(rust_vec[128 * 9_999..128 * 10_000], SIFT_10K_LAST_VEC_F32);
}

#[test]
fn test_convert_sift_learn_fbin_to_u8bin() {
    let sift_learn_f32_path = format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        SIFT_10K_FBIN_FILENAME
    );
    let sift_learn_u8_path = format!(
        "{}/{}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        SIFT_10K_U8BIN_FILENAME
    );

    let convert_ret = convert_fbin_to_u8bin(sift_learn_f32_path, sift_learn_u8_path.clone());
    assert!(convert_ret.is_ok());

    let (metadata, rust_vec) = rust_read_file::<u8>(sift_learn_u8_path).unwrap();
    assert_eq!(metadata.npts_u32, 10_000);
    assert_eq!(metadata.ndims_u32, 128);
    assert_eq!(
        rust_vec.len(),
        metadata.npts_u32 as usize * metadata.ndims_u32 as usize
    );

    // The following two tests hold for SIFT_LEARN because it's really a u8 dataset padded up to f32
    // This test is not valid for other datasets in general.
    assert_eq!(
        rust_vec[0..128]
            .iter()
            .map(|&x| x as f32)
            .collect::<Vec<f32>>(),
        SIFT_10K_FIRST_VEC_F32
    );
    assert_eq!(
        rust_vec[128 * 9_999..128 * 10_000]
            .iter()
            .map(|&x| x as f32)
            .collect::<Vec<f32>>(),
        SIFT_10K_LAST_VEC_F32
    );
}

#[test]
fn test_create_empty_f32bin_rw() {
    let empty_f32bin_path = format!(
        "{}/empty.fbin",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    let cfilename = CString::new(empty_f32bin_path.clone()).unwrap();
    let filename = String::from(empty_f32bin_path);
    let metadata = Metadata {
        npts_u32: 10,
        ndims_u32: 3,
    };
    let ret = unsafe { create_empty_f32bin_of_size(cfilename.as_bytes().as_ptr(), &metadata) };
    assert_eq!(ret, 0);

    // Read the file and make sure it's zeros
    let (metadata, rust_vec) = rust_read_file::<f32>(filename).unwrap();
    assert_eq!(metadata.npts_u32, 10);
    assert_eq!(metadata.ndims_u32, 3);
    assert_eq!(
        rust_vec.len(),
        metadata.npts_u32 as usize * metadata.ndims_u32 as usize
    );
    assert_eq!(rust_vec.iter().all(|&x| x == 0.0), true);

    // Write some data to the file and read it back
    let mut fp: *mut std::os::raw::c_void = std::ptr::null_mut();
    let ret = unsafe { open_fp_to_rw(cfilename.as_bytes().as_ptr(), &mut fp) };
    assert_eq!(ret, 0);
    let first_vec = vec![1.0, 2.0, 3.0];
    let last_vec = vec![4.0, 5.0, 6.0];
    let ret = unsafe { set_vector_f32(fp, &metadata, 0, first_vec.as_ptr()) };
    assert_eq!(ret, 0);
    let ret = unsafe { set_vector_f32(fp, &metadata, 9, last_vec.as_ptr()) };
    assert_eq!(ret, 0);

    let mut vec = vec![0.0; metadata.ndims_u32 as usize];
    let ret = unsafe { get_vector_f32(fp, &metadata, 0, vec.as_mut_ptr()) };
    assert_eq!(ret, 0);
    assert_eq!(vec, first_vec);
    let ret = unsafe { get_vector_f32(fp, &metadata, 9, vec.as_mut_ptr()) };
    assert_eq!(ret, 0);
    assert_eq!(vec, last_vec);

    let mut final_metadata = Metadata::default();
    let res = unsafe { read_metadata(fp, &mut final_metadata) };
    assert_eq!(res, 0);
    assert_eq!(final_metadata.npts_u32, 10);
    assert_eq!(final_metadata.ndims_u32, 3);

    let drop_ret = unsafe { drop_fp(fp) };
    assert_eq!(drop_ret, 0);
}

#[test]
fn test_c_vec_provider_u8() {
    let empty_f32bin_path = format!(
        "{}/empty.u8bin",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    let filename = String::from(empty_f32bin_path.clone());
    let metadata = Metadata {
        npts_u32: 10,
        ndims_u32: 3,
    };

    let cvec_provider = CVecProviderU8::new(filename, Some(metadata));
    assert_eq!(cvec_provider.metadata.npts_u32, metadata.npts_u32);
    assert_eq!(cvec_provider.metadata.ndims_u32, metadata.ndims_u32);

    let first_vec = cvec_provider.get_vector_u8(0).unwrap();
    assert_eq!(first_vec.iter().all(|&x| x == 0), true);

    let last_vec = vec![1u8, 2, 3];
    cvec_provider.set_vector_u8(9, &last_vec).unwrap();
    let last_vec_read = cvec_provider.get_vector_u8(9).unwrap();
    assert_eq!(last_vec, last_vec_read);

    cvec_provider.drop();
}
