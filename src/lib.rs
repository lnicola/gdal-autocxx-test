use std::ffi::CStr;
use std::mem;
use std::pin::Pin;
use std::ptr;

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

    subclass!("GDALDataset", MyDataset)

    generate!("upcast_driver")
    generate!("set_driver_functions")
}

use ffi::*;

#[subclass]
#[derive(Default)]
pub struct MyDataset;

impl GDALDataset_methods for MyDataset {}

impl CppPeerConstructor<MyDatasetCpp> for MyDataset {
    fn make_peer(
        &mut self,
        _peer_holder: CppSubclassRustPeerHolder<Self>,
    ) -> UniquePtr<MyDatasetCpp> {
        todo!()
    }
}

pub extern "C" fn open(_: *mut GDALOpenInfo) -> *mut GDALDataset {
    println!("Hello from open");
    ptr::null_mut()
}

pub extern "C" fn identify(_: *mut GDALOpenInfo) -> i32 {
    println!("Hello from identify");
    0
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

        ffi::set_driver_functions(
            &mut *Pin::<&mut GDALDriver>::into_inner_unchecked(driver.as_mut().unwrap()),
            mem::transmute(&open),
            mem::transmute(&identify),
        );

        let driver_manager = Pin::new_unchecked(&mut *GetGDALDriverManager());
        driver_manager.RegisterDriver(driver.into_raw());
    }
}
