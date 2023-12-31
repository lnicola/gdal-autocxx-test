use std::ffi::CStr;
use std::mem;
use std::pin::Pin;
use std::ptr;
use std::slice;

use autocxx::prelude::*;
use autocxx::subclass::*;

include_cpp! {
    #include "gdal/gdal.h"
    #include "gdal/gdal_priv.h"

    #include "utils.h"

    safety!(unsafe_ffi)

    generate!("GDALMajorObject")
    generate!("GDALDriverManager")
    generate!("GDALDriver")
    generate!("GDALDataset")
    generate!("GDAL_DCAP_RASTER")
    generate!("GDAL_DMD_LONGNAME")
    generate!("GetGDALDriverManager")
    generate!("GDALOpenInfo")
    generate!("GDALRasterBand")

    subclass!("GDALDataset", MyDataset)
    subclass!("GDALRasterBand", MyRasterBand)

    generate!("upcast_driver")
    generate!("gdal_driver_set_functions")
    generate!("gdal_open_info_get_filename")
    generate!("gdal_open_info_get_header_bytes")
    generate!("gdal_open_info_get_header")
}

use ffi::*;

#[subclass]
#[derive(Default)]
pub struct MyDataset;

impl GDALDataset_methods for MyDataset {}

impl CppPeerConstructor<MyDatasetCpp> for MyDataset {
    fn make_peer(
        &mut self,
        peer_holder: CppSubclassRustPeerHolder<Self>,
    ) -> UniquePtr<MyDatasetCpp> {
        UniquePtr::emplace(MyDatasetCpp::new(peer_holder))
    }
}

#[subclass]
#[derive(Default)]
pub struct MyRasterBand;

impl GDALRasterBand_methods for MyRasterBand {
    unsafe fn IReadBlock(
        &mut self,
        nBlockXOff: c_int,
        nBlockYOff: c_int,
        pData: *mut c_void,
    ) -> CPLErr {
        CPLErr::CE_Failure
    }
}

impl CppPeerConstructor<MyRasterBandCpp> for MyRasterBand {
    fn make_peer(
        &mut self,
        peer_holder: CppSubclassRustPeerHolder<Self>,
    ) -> UniquePtr<MyRasterBandCpp> {
        UniquePtr::emplace(MyRasterBandCpp::new(peer_holder))
    }
}

pub extern "C" fn open(open_info: *mut GDALOpenInfo) -> *mut GDALDataset {
    println!("Hello from open");
    let filename = unsafe { gdal_open_info_get_filename(open_info) };
    let filename = unsafe { CStr::from_ptr(filename) };
    println!("open filename: {:?}", filename);

    if identify(open_info).0 == 0 {
        return ptr::null_mut();
    }
    let mut dataset = MyDataset::new_cpp_owned(MyDataset::default());
    {
        let dataset = dataset.pin_mut();
        let ds = dataset.As_GDALDataset_mut();

        let rasterband = MyRasterBand::new_cpp_owned(MyRasterBand::default()).into_raw();
        unsafe {
            let band = mem::transmute::<_, *mut GDALRasterBand>(rasterband);
            ds.SetBand(1.into(), band);
        }
    }
    dataset.into_raw() as *mut GDALDataset
}

pub extern "C" fn identify(open_info: *mut GDALOpenInfo) -> c_int {
    println!("Hello from identify");
    let filename = unsafe { gdal_open_info_get_filename(open_info) };
    let filename = unsafe { CStr::from_ptr(filename) };
    let header_bytes = unsafe { gdal_open_info_get_header_bytes(open_info).0 };
    let header_ptr = unsafe { gdal_open_info_get_header(open_info) };
    let header = unsafe { slice::from_raw_parts(header_ptr, header_bytes as usize) };
    println!("identify filename: {:?}", filename);
    println!("header bytes: {:x?}", header);
    c_int::from(1)
}

#[no_mangle]
pub extern "C" fn GDALRegisterMe() {
    println!("Hello from GDALRegisterMe");
    let mut driver = GDALDriver::new().within_unique_ptr();
    unsafe {
        {
            let driver = driver.as_mut().unwrap();
            let driver = Pin::<&mut GDALDriver>::into_inner_unchecked(driver);
            let driver = ffi::upcast_driver(&mut *driver);
            let driver = Pin::new_unchecked(&mut *driver);
            driver.SetDescription(CStr::from_bytes_with_nul(b"MyDriver\0").unwrap().as_ptr());
        }

        {
            driver.as_mut().unwrap().SetMetadataItem(
                GDAL_DCAP_RASTER.as_ptr() as _,
                CStr::from_bytes_with_nul(b"YES\0").unwrap().as_ptr(),
                ptr::null(),
            );
            driver.as_mut().unwrap().SetMetadataItem(
                GDAL_DMD_LONGNAME.as_ptr() as _,
                CStr::from_bytes_with_nul(b"Rust Raster Test Driver\0")
                    .unwrap()
                    .as_ptr(),
                ptr::null(),
            );
        }

        ffi::gdal_driver_set_functions(
            &mut *Pin::<&mut GDALDriver>::into_inner_unchecked(driver.as_mut().unwrap()),
            mem::transmute(open as extern "C" fn(*mut GDALOpenInfo) -> *mut GDALDataset),
            mem::transmute(identify as extern "C" fn(*mut GDALOpenInfo) -> c_int),
        );

        let driver_manager = Pin::new_unchecked(&mut *GetGDALDriverManager());
        driver_manager.RegisterDriver(driver.into_raw());
    }
}
