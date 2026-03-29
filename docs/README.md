# xwsystem documentation

This folder holds project documentation. The long narrative doc that used to live here duplicated [README.md](../README.md) and [README_LONG.md](../README_LONG.md) with outdated install extras; use those files and the index below instead.

## Start here

- **[INDEX.md](INDEX.md)** - document map and quick links  
- **[GUIDE_01_USAGE.md](GUIDE_01_USAGE.md)** - install modes (Lite / Lazy / Full), codecs, caching, runtime  

## Reference and status

- [REF_01_REQ.md](REF_01_REQ.md), [REF_22_PROJECT.md](REF_22_PROJECT.md)  
- [REF_15_API.md](REF_15_API.md), [REF_13_ARCH.md](REF_13_ARCH.md)  
- [REF_14_DX.md](REF_14_DX.md), [REF_50_QA.md](REF_50_QA.md), [REF_54_BENCH.md](REF_54_BENCH.md), [REF_51_TEST.md](REF_51_TEST.md)  

## Other areas

- [compliance/](compliance/) - standards and evidence layout  
- [logs/](logs/) - change logs, test summaries, benchmarks where recorded  

Install today: `pip install exonware-xwsystem`, `pip install exonware-xwsystem[lazy]`, or `pip install exonware-xwsystem[full]` (Python 3.12+). The `[all]` extra described in older copies of this file is not the supported split; see `pyproject.toml` for the current optional dependency groups.
