UI:
- [x] ~~Implement plotting/hiding of groups accessible with switch
      buttons on plotting UI~~
- [.] Include legend
  - [x] ~Showing filename~~
  - [x] ~~Showing alias~~
- [x] ~~Select good colors for plots based on FileID~~
- [ ] Implement file settings form
  - [x] ~~Use collapsing header for file settings form~~
  - [x] ~~Allow alias for file~~
  - [ ] Allow selection of plot color
  - [ ] Implement scaling/shifting of plots (form based and mouse control)
  - [ ] Implement plotting of columns different than first two
    - [ ] Allow plotting more than one dataset from CSV file
- [ ] Create Global Settings View
- [ ] Include error log window 
- [ ] Files in file settings view should have a fixed order

- [ ] Allow moving files between groups

Search:
- [x] ~~Do not block the UI when opening new search path via file dialog~~
- [ ] Allow search selection using keyboard up/down

Plotter:
- [ ] Have a flag to disable/enable the legend

Internals:
- [x] ~~Use array of length 10 for groups instead of hashmap~~
- [ ] Handle all UI interactions using an event system?
- [ ] Remove hard coded paths
  - [ ] Work with configuration file