#pragma once

#include <gdal/gdal.h>
#include <gdal/gdal_priv.h>

inline GDALMajorObject *upcast_driver(GDALDriver *driver) { return driver; }

inline void gdal_driver_set_functions(GDALDriver *driver, void *pfnOpen,
                                      void *pfnIdentify) {
  driver->pfnOpen = reinterpret_cast<GDALDataset *(*)(GDALOpenInfo *)>(pfnOpen);
  driver->pfnIdentify = reinterpret_cast<int (*)(GDALOpenInfo *)>(pfnIdentify);
}

inline char *gdal_open_info_get_filename(const GDALOpenInfo *openInfo) {
  return openInfo->pszFilename;
}

inline int gdal_open_info_get_header_bytes(const GDALOpenInfo *openInfo) {
  return openInfo->nHeaderBytes;
}

inline unsigned char *gdal_open_info_get_header(const GDALOpenInfo *openInfo) {
  return openInfo->pabyHeader;
}
