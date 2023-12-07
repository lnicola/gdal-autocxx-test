#pragma once

#include <gdal/gdal.h>
#include <gdal/gdal_priv.h>

inline GDALMajorObject *upcast_driver(GDALDriver *driver) { return driver; }

inline void set_driver_pfnOpen(GDALDriver *driver, void *pfnOpen) {
  driver->pfnOpen = reinterpret_cast<GDALDataset *(*)(GDALOpenInfo *)>(pfnOpen);
}
