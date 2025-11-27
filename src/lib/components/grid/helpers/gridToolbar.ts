export function autoResize(api) {
  api.sizeColumnsToFit();
}

export function exportCSV(api) {
  api.exportDataAsCsv();
}

export function openFilters(api) {
  api.setSideBarVisible(true);
}
