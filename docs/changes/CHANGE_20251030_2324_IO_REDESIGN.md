# IO Module Redesign Proposal

**Following GUIDELINES_DEV.md Design Patterns**

## ?? Current Issues

### 1. Manager Folder Ambiguity
```
manager/
+-- file_manager.py   # Is this needed? XWFile already handles files
+-- unified_io.py     # This is valuable - it's our FACADE!
```

**Questions:**
- Is `XWFileManager` redundant if `XWFile` is comprehensive?
- Should `unified_io.py` be at root as the main facade?
- Can we eliminate `manager/` folder by better design?

### 2. Archive Design Misalignment
```
archive/
+-- archive.py        # Creates/extracts archives - this is a PROCESSOR!
+-- compression.py    # Compresses/decompresses - also a PROCESSOR!
```

**Should be:**
- Archive PROCESSOR ? Use `ICodec` pattern (encode=create, decode=extract)
- Archive FILE ? Extend `XWFile` (for working with .zip files as files)

### 3. Class Naming Confusion
**CORRECTION NEEDED:**
- File name: `file.py` ?
- Class name: `XWFile` (NOT `File`) ?

## ??? **REDESIGN OPTIONS**

### **Option 1: Facade Pattern (Recommended)**

**Following GUIDELINES_DEV.md - Facade is MANDATORY for libraries**

```
io/
+-- contracts.py      # All interfaces
+-- defs.py          # All enums
+-- errors.py        # All exceptions
+-- base.py          # Root bases
+-- facade.py        # ? NEW: Main facade (XWUnifiedIO here!)
+-- __init__.py      # Exports facade
�
+-- codec/           # Codec system
+-- common/          # Shared utilities
+-- file/            # File operations (XWFile, XWFileManager?)
+-- folder/          # Folder operations (XWFolder)
+-- stream/          # Stream + codec integration
+-- filesystem/      # Virtual FS
+-- archive/         # Should use ICodec pattern!
```

**Benefits:**
- ? `facade.py` = Main entry point (like GUIDELINES says)
- ? `XWUnifiedIO` becomes the facade
- ? No confusing `manager/` folder
- ? Clean separation

### **Option 2: Root-Level Facade**

```
io/
+-- contracts.py
+-- defs.py
+-- errors.py
+-- base.py
+-- unified_io.py    # ? At root as main facade
+-- __init__.py
�
+-- file/
�   +-- file.py      # XWFile
�   +-- file_manager.py  # XWFileManager (if needed)
+-- folder/
�   +-- folder.py    # XWFolder
+-- ...
```

### **Option 3: Eliminate FileManager**

**Question:** Do we need `XWFileManager`?

**Comparison:**
```python
# XWFile - Already has:
file = XWFile("data.txt")
file.save(data)
file.load()
file.open()
file.read()
# + atomic operations via AtomicFileWriter
# + validation via PathValidator

# XWFileManager - What extra does it provide?
manager = XWFileManager("base/path")
manager.save(data, "file.txt")  # Same as XWFile?
manager.load("file.txt")        # Same as XWFile?
```

**If XWFile is comprehensive, do we need FileManager?**

### **Option 4: Codec-Based Archives**

**Current (Wrong):**
```python
class Archive:
    def create(self, source, archive_path): ...
    def extract(self, archive_path, dest): ...
```

**Should be (Following ICodec):**
```python
class ZipCodec(ACodec[Path, bytes]):
    """Zip archive as a codec."""
    
    def encode(self, source_path: Path) -> bytes:
        """Encode folder ? zip bytes."""
        ...
    
    def decode(self, zip_bytes: bytes) -> Path:
        """Decode zip bytes ? extracted folder."""
        ...
```

**Or for files:**
```python
class XWArchiveFile(XWFile):
    """Archive file (extends XWFile)."""
    
    def extract_to(self, dest: Path): ...
    def add_file(self, file: Path): ...
```

## ?? **RECOMMENDED DESIGN**

Based on GUIDELINES_DEV.md principles:

### **Structure:**
```
io/
+-- contracts.py      # ALL interfaces (1 file)
+-- defs.py          # ALL enums (1 file)
+-- errors.py        # ALL exceptions (1 file)
+-- base.py          # Root bases
+-- facade.py        # XWUnifiedIO - Main facade (MANDATORY)
+-- __init__.py      # Exports facade + components
�
+-- codec/           # Codec abstractions
�   +-- base.py
�
+-- common/          # Shared utilities
�   +-- base.py
�   +-- atomic.py    # AtomicFileWriter
�   +-- watcher.py   # XWFileWatcher
�   +-- lock.py      # XWFileLock
�
+-- file/            # File operations
�   +-- base.py
�   +-- file.py      # XWFile (main file class)
�   +-- source.py    # FileDataSource
�   +-- paging/      # Paging strategies
�
+-- folder/          # Folder operations
�   +-- base.py
�   +-- folder.py    # XWFolder (main folder class)
�
+-- stream/          # Stream + codec integration
�   +-- base.py
�   +-- codec_io.py  # XWCodecIO
�   +-- async_operations.py
�
+-- filesystem/      # Virtual FS
�   +-- base.py
�   +-- local.py     # XWLocalFileSystem
�
+-- archive/         # Archive CODECS (not file operations!)
    +-- base.py      # AArchiveCodec(ACodec)
    +-- zip_codec.py # XWZipCodec(AArchiveCodec)
    +-- tar_codec.py # XWTarCodec(AArchiveCodec)
```

### **Key Changes:**

1. **facade.py at root** (MANDATORY per GUIDELINES)
   - `XWUnifiedIO` lives here
   - Main entry point for all IO operations
   - Delegates to specialized components

2. **No manager/ folder**
   - `XWFileManager` eliminated (XWFile is enough)
   - Or moved to `file/` if really needed

3. **Archive as Codecs**
   - `XWZipCodec` - Encode folder to zip, decode zip to folder
   - Follows ICodec pattern
   - Integrates with codec registry

4. **Consistent XW prefix**
   - `XWFile`, `XWFolder`, `XWUnifiedIO`
   - File names: `file.py`, `folder.py`, `facade.py`

## ?? **Questions for You:**

1. **FileManager:** Keep it? Move to `file/`? Or eliminate?

2. **Manager folder:** Remove it entirely?

3. **UnifiedIO location:** 
   - Option A: `facade.py` at root (follows GUIDELINES)
   - Option B: Keep in manager/ folder
   - Option C: Directly in root as `unified_io.py`

4. **Archive pattern:**
   - Option A: Codec pattern (encode/decode)
   - Option B: Keep current (create/extract)
   - Option C: Both (codec for processing, XWArchiveFile for file operations)

5. **Class naming:**
   - All concrete classes: XW prefix (XWFile, XWFolder, etc.)
   - File names: No xw_ prefix (file.py, folder.py, etc.)
   - Correct?

## ?? **My Recommendation:**

```python
# Public API (following facade pattern)
from exonware.xwsystem.io import XWUnifiedIO

# Main facade - delegates to specialized components
io = XWUnifiedIO()

# File operations (delegates to XWFile internally)
io.save_file("data.txt", content)
io.load_file("data.txt")

# Folder operations (delegates to XWFolder internally)
io.create_folder("output/")
io.list_files("output/")

# Archive operations (uses codec pattern internally)
io.archive_folder("backup/", "backup.zip", codec="zip")
io.extract_archive("backup.zip", "restore/")

# Codec integration
io.save_with_codec(data, "output.json", codec="json")
io.load_with_codec("output.json", codec="json")
```

**This follows:**
- ? Facade pattern (MANDATORY)
- ? Single entry point
- ? Clean delegation
- ? Strategy pattern (different codecs)
- ? All 5 priorities

What do you think?


