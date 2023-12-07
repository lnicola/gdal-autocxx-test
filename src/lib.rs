use std::ffi::CStr;
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
    generate!("GetGDALDriverManager")

    subclass!("GDALDataset", MyDataset)

    generate!("upcast_driver")
    generate!("set_driver_pfnOpen")
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

#[no_mangle]
pub extern "C" fn GDALRegister_My() {
    let mut driver = GDALDriver::new().within_box();
    unsafe {
        {
            let driver = driver.as_mut();
            let driver = Pin::<&mut GDALDriver>::into_inner_unchecked(driver);
            let driver = ffi::upcast_driver(&mut *driver);
            let driver = Pin::new_unchecked(&mut *driver);
            driver.SetDescription(CStr::from_bytes_with_nul(b"MyDriver\0").unwrap().as_ptr());
        }

        {
            driver.as_mut().SetMetadataItem(
                GDAL_DCAP_RASTER.as_ptr() as _,
                CStr::from_bytes_with_nul(b"YES\0").unwrap().as_ptr(),
                ptr::null(),
            );
        }

        let driver_manager = Pin::new_unchecked(&mut *GetGDALDriverManager());
        driver_manager.RegisterDriver(Pin::<&mut GDALDriver>::into_inner_unchecked(
            driver.as_mut(),
        ));
        ffi::set_driver_pfnOpen(
            &mut *Pin::<&mut GDALDriver>::into_inner_unchecked(driver.as_mut()),
            ptr::null_mut(),
        );
    }
}
