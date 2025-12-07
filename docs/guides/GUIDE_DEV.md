# eXonware Development Guide

**Company:** eXonware.com
**Author:** Eng. Muhammad AlShehri
**Email:** connect@exonware.com
**Version:** 0.0.1.410
**Generation Date:** 07-Sep-2025

## 📋 AI-Friendly Document

**This document is designed for both human developers and AI assistants.** All guidelines, rules, and principles must be followed for ANY work - not just code, but documentation, testing, project structure, versioning, and all deliverables. Use this as your comprehensive quality standard.

**Related Documents:**
- [GUIDE_ARCH.md](GUIDE_ARCH.md) - Architecture playbook (layering, roadmap, compliance)
- [GUIDE_MASTER.md](GUIDE_MASTER.md) - Master standards and shared constraints
- [GUIDE_COMP.md](GUIDE_COMP.md) - Compliance program execution
- This document (GUIDE_DEV.md) - Core development philosophy and standards
- **[GUIDE_TEST.md](GUIDE_TEST.md)** - Detailed testing implementation and runner architecture
- **[GUIDE_DOCS.md](GUIDE_DOCS.md)** - Documentation standards and best practices

## Table of Contents

1. [Core Development Philosophy](#core-development-philosophy)
2. [Project Structure & Organization](#project-structure--organization)
3. [Code Quality Standards](#code-quality-standards)
4. [Testing Strategy](#testing-strategy)
5. [Documentation Standards](#documentation-standards)
6. [Version Management](#version-management)
7. [Import & Dependency Management](#import--dependency-management)
8. [eXonware Ecosystem Architecture](#exonware-ecosystem-architecture)
9. [Project Categories & Standards](#project-categories--standards)
10. [Security & Performance](#security--performance)
11. [Code Review & CI/CD Standards](#code-review--cicd-standards)
12. [AI Development Guidelines](#ai-development-guidelines)
13. [Library-Specific Patterns](#library-specific-patterns)
14. [Core-Facade Development Strategy](#core-facade-development-strategy)
15. [Development Environment](#development-environment)
16. [Release & Publishing](#release--publishing)

---

## Core Development Philosophy

### Priority Order
1. **Security** - Systems must be secure by design, protecting users and data
2. **Usability** - Systems must be easy to use and intuitive
3. **Maintainability** - Code should be clean, well-structured, and easy to maintain
4. **Performance** - Systems should be efficient and fast
5. **Extensibility** - Code should be designed to be easily extended and modified

### Core Principles
- **Never remove features** - Always preserve existing functionality; never take out features that are already implemented
- **Never reinvent the wheel** - Reuse code from exonware libraries (especially xwsystem) or external Python libs (if unique size is <1.5MB) to leverage proven solutions and reduce maintenance burden
- **Think and design thoroughly** - Spend more time thinking and designing features rather than writing extensive code to prevent architectural debt and ensure long-term maintainability
- **Simple, concise solutions** - Value simple, concise solutions when implementing features to reduce complexity and improve developer productivity
- ** quality** - Build long-term, clean, extensible, maintainable code to ensure enterprise readiness and reduce technical debt
- **Challenge ideas** - Prefer challenging rather than just agreeing to ensure robust solutions and avoid groupthink
- **Fix root causes** - Never remove features; always resolve root causes instead of using workarounds to maintain system integrity and prevent technical debt accumulation
- **Never permanently delete files** - Move to '_delete' folder if removal needed to maintain audit trail and enable recovery if needed
- **Include full file path at the top commented** - Always add the full file path as a comment at the top of every file for better traceability: #exonware/xwsystem/... etc

### Error Fixing Philosophy

**⚠️ CRITICAL: Root Cause Analysis is MANDATORY**

When fixing errors, bugs, or test failures, you **MUST** follow this approach:

#### **The 5-Priority Root Cause Method**

Every fix must be evaluated against eXonware's 5 core priorities in order:

1. **Security First** (Priority #1)
 - Is this error exposing a security vulnerability?
 - Does the fix introduce new security risks?
 - Have we validated input/output security?
 - Are we following OWASP Top 10 guidelines?

2. **Usability Impact** (Priority #2)
 - Does the fix improve user experience?
 - Are error messages clear and helpful?
 - Is the API still intuitive after the fix?
 - Have we considered developer experience?

3. **Maintainability** (Priority #3)
 - Is the fix clean and well-structured?
 - Does it follow design patterns?
 - Will future developers understand this code?
 - Have we added proper documentation?

4. **Performance** (Priority #4)
 - Does the fix maintain or improve performance?
 - Have we benchmarked the solution?
 - Are we introducing performance regressions?
 - Is the solution efficient?

5. **Extensibility** (Priority #5)
 - Does the fix maintain extensibility?
 - Can this be extended in the future?
 - Have we used proper abstractions?
 - Is the solution flexible?

#### **Forbidden Error Fixing Anti-Patterns**

**? NEVER DO THESE:**

1. **Don't use `pass` to silence errors:**
 ```python
 # ? BAD: Hiding the problem
 try:
 complex_operation()
 except Exception:
 pass # Error hidden, problem persists!
 ```

2. **Don't remove features to fix bugs:**
 ```python
 # ? BAD: Removing functionality
 # def problematic_feature():
 # # Commented out because it had a bug
 # pass
 ```

3. **Don't use workarounds instead of fixes:**
 ```python
 # ? BAD: Working around the issue
 if not is_broken():
 do_operation()
 # Should fix why it's broken, not skip it
 ```

4. **Don't rig tests to pass:**
 ```python
 # ? BAD: Test always passes
 def test_important_feature():
 try:
 important_feature()
 assert True # Meaningless assertion
 except:
 pass # Swallows failures
 ```

5. **Don't ignore error messages:**
 ```python
 # ? BAD: Suppressing warnings
 import warnings
 warnings.filterwarnings("ignore") # Hiding problems
 ```

6. **Don't use generic exception handling:**
 ```python
 # ? BAD: Catching everything
 try:
 operation()
 except: # Catches ALL exceptions, even KeyboardInterrupt
 return None
 ```

7. **Don't hide errors with pytest flags:**
 ```bash
 # ? BAD: Hiding warnings and errors
 pytest --disable-warnings # Hides real problems!
 pytest --maxfail=10 # Continues after failures, masks issues!
 pytest --tb=no # Hides tracebacks!
 pytest -q # Too quiet, hides important info!
 ```

8. **Don't suppress warnings in configuration:**
 ```ini
 # ? BAD: pytest.ini
 [tool:pytest]
 addopts = --disable-warnings # Hiding problems!
 filterwarnings = ignore # Suppressing all warnings!
 ```

9. **Don't skip tests to avoid failures:**
 ```python
 # ? BAD: Avoiding the problem
 @pytest.mark.skip("Broken, will fix later")
 def test_critical_feature():
 pass

 @pytest.mark.xfail(reason="Known issue")
 def test_important_validation():
 pass
 ```

10. **Don't use environment variables to hide issues:**
 ```bash
 # ? BAD: Hiding warnings
 export PYTHONWARNINGS=ignore
 export PYTEST_DISABLE_PLUGIN_AUTOLOAD=1
 ```

#### **Forbidden pytest Flags & Configurations**

**❌ NEVER USE THESE - They hide real problems:**

**See [GUIDE_TEST.md - Forbidden pytest Flags](GUIDE_TEST.md#forbidden-pytest-flags--configurations) for complete list and explanations.**

**Key forbidden flags:**
- `--disable-warnings` - Hides warnings that indicate real issues
- `--maxfail=10` - Continues after failures, masks cascading issues
- `--tb=no` - Hides tracebacks needed for debugging
- `@pytest.mark.skip` - Avoids running tests instead of fixing them

**? ALLOWED:** `-v`, `--tb=short`, `--maxfail=1` or `-x`, `--strict-markers`, `-m`, `--lf`, `--ff`, `--cov`, `-s`, `-l`

#### **Correct Error Fixing Approach**

**? ALWAYS DO THIS:**

1. **Identify root cause:**
 ```python
 # ? GOOD: Understanding the problem
 def fix_validation_error():
 # Root cause: Email validation regex doesn't handle subdomains
 # Fix: Update regex pattern to be more comprehensive
 pattern = r'^[^@]+@[^@]+\.[^@]+' # Old
 pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$' # Fixed
 ```

2. **Fix the actual problem:**
 ```python
 # ? GOOD: Fixing the issue
 def process_data(data):
 # Root cause: Data can be None
 # Fix: Add proper validation
 if data is None:
 raise ValueError("Data cannot be None")
 return transform(data)
 ```

3. **Add proper error handling:**
 ```python
 # ? GOOD: Specific exception handling
 try:
 result = parse_config(file_path)
 except FileNotFoundError:
 logger.error(f"Config file not found: {file_path}")
 raise ConfigurationError(f"Missing required config: {file_path}")
 except yaml.YAMLError as e:
 logger.error(f"Invalid YAML in {file_path}: {e}")
 raise ConfigurationError(f"Invalid config format: {e}")
 ```

4. **Write tests that prevent regression:**
 ```python
 # ? GOOD: Test the fix
 def test_email_validation_handles_subdomains():
 """Ensure email validation supports subdomain addresses."""
 # This test failed before the fix
 assert validate_email("user@mail.company.com") == True
 assert validate_email("admin@internal.corp.example.com") == True
 ```

5. **Document WHY the fix was needed:**
 ```python
 def validate_email(email: str) -> bool:
 """
 Validate email address format.

 Root cause fixed: Previous regex didn't support multi-level subdomains
 (e.g., user@mail.company.com). Updated to RFC 5322 compliance.

 Priority alignment:
 - Security: Prevents injection via email field
 - Usability: Accepts valid email formats users expect
 - Maintainability: Clear regex pattern with explanation
 """
 pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
 return bool(re.match(pattern, email))
 ```

#### **Error Fixing Workflow**

**Step-by-step process:**

1. **Reproduce the error** - Understand exactly when/how it occurs
2. **Analyze root cause** - Don't stop at symptoms, find the real problem
3. **Evaluate against 5 priorities** - How does the fix impact each priority?
4. **Design proper solution** - Address root cause, not symptoms
5. **Implement with tests** - Write tests that would have caught this
6. **Document the fix** - Explain WHY this was needed
7. **Verify no regressions** - Run full test suite
8. **Review against priorities** - Final check against all 5 priorities

#### **Examples: Right vs Wrong**

**Scenario: Test fails because function returns None unexpectedly**

? **Wrong Approach:**
```python
# Just make the test pass
def test_get_user():
 user = get_user("test@example.com")
 if user is None:
 pass # Ignore the failure
 else:
 assert user.email == "test@example.com"
```

? **Right Approach:**
```python
# Fix the root cause: get_user() should never return None for valid emails
def get_user(email: str) -> User:
 """
 Get user by email.

 Root cause fixed: Function returned None when user not in cache.
 Now properly queries database as fallback.

 Raises:
 UserNotFoundError: If user doesn't exist in system
 """
 # Check cache first
 user = cache.get(email)
 if user:
 return user

 # Fallback to database
 user = db.query_user(email)
 if user is None:
 raise UserNotFoundError(f"User not found: {email}")

 # Cache for next time
 cache.set(email, user)
 return user

def test_get_user_from_database():
 """Test that get_user queries database when not in cache."""
 clear_cache() # Ensure not in cache
 user = get_user("test@example.com")
 assert user.email == "test@example.com"

def test_get_user_raises_when_not_exists():
 """Test that get_user raises proper error for non-existent users."""
 with pytest.raises(UserNotFoundError):
 get_user("nonexistent@example.com")
```

**Why this matters:**
- ? **Security**: Proper error handling prevents information leakage
- ? **Usability**: Clear errors help developers debug issues
- ? **Maintainability**: Future developers understand the logic
- ? **Performance**: Caching strategy is clear and efficient
- ? **Extensibility**: Easy to add more data sources

---

---

## Project Structure & Organization

### Standard Directory Structure

**MANDATORY: Library Root Directory Structure**

For all eXonware library-type projects (`xwsystem`, `xwnode`, `xwquery`, `xwdata`, `xwschema`, `xwaction`, `xwentity`), the root directory **MUST contain ONLY the following files and folders**:

```
library-name/ # Root directory
+-- .github/ # GitHub configuration
� +-- workflows/ # CI/CD workflows
+-- .gitignore # Git ignore rules
+-- LICENSE # MIT License
+-- MANIFEST.in # Package manifest (optional)
+-- pyproject.toml # Main package configuration
+-- pyproject.<library>.toml # Convenience wrapper configuration
+-- pytest.ini # Pytest configuration
+-- README.md # Main README (ONLY one in root)
+-- requirements.txt # Dependencies list
+-- docs/ # ALL documentation (except main README.md)
+-- examples/ # Usage examples
+-- src/ # Source code
� +-- exonware/
� � +-- library_name/ # Main package
� � +-- __init__.py # Package initialization
� � +-- contracts.py # Enums and interfaces (REQUIRED)
� � +-- errors.py # Module-specific errors (REQUIRED, file not folder)
� � +-- base.py # Abstract classes (REQUIRED)
� � +-- facade.py # Facade pattern implementation (REQUIRED)
� � +-- defs.py # Type definitions and constants (REQUIRED)
� � +-- config.py # Configuration classes (REQUIRED)
� � +-- version.py # Version information (REQUIRED)
� � +-- [modules]/ # Feature modules (as needed)
� � +-- __init__.py
� � +-- contracts.py # Module enums/interfaces (OPTIONAL)
� � +-- errors.py # Module errors (OPTIONAL)
� � +-- base.py # Module abstract classes (OPTIONAL)
� � +-- facade.py # Module facade (OPTIONAL)
� � +-- defs.py # Module definitions (OPTIONAL)
� � +-- config.py # Module config (OPTIONAL)
� +-- library_name.py # Convenience alias
+-- tests/ # Test suite
 +-- __init__.py
 +-- conftest.py
 +-- runner.py # Main test runner
 +-- verify_installation.py
 +-- 0.core/ # Core functionality tests
 +-- 1.unit/ # Unit tests
 +-- 2.integration/ # Integration tests
 +-- 3.advance/ # Advance tests (v1.0.0+)
```

**⚠️ CRITICAL RULES - Library Root Directory:**

**? ALLOWED at root level (ONLY these):**
1. `.github/` - GitHub configuration folder
2. `.gitignore` - Git ignore rules
3. `LICENSE` - MIT License file
4. `MANIFEST.in` - Package manifest (optional)
5. `pyproject.toml` - Main package configuration
6. `pyproject.<library>.toml` - Convenience wrapper configuration
7. `pytest.ini` - Pytest configuration
8. `README.md` - Main README (ONLY one allowed in root)
9. `requirements.txt` - Dependencies list
10. `docs/` - Documentation folder
11. `examples/` - Examples folder
12. `src/` - Source code folder
13. `tests/` - Test suite folder

**? NOT ALLOWED at root level:**
- No other markdown files (all must be in `docs/`)
- No loose Python scripts
- No data files
- No log files
- No backup folders
- No temporary files
- No nested documentation
- No additional configuration files
- No session/summary files
- No migration folders (use internal structure)
- **Exception**: Hidden files like `.vscode/`, `.idea/` (IDE-specific, gitignored)

**Why this matters:**
- **Clean repository** - Easy to navigate and understand
- **Consistent structure** - All libraries follow same pattern
- **Professional presentation** - Enterprise-grade organization
- **Maintainability** - Clear separation of concerns
- **CI/CD compatibility** - Standard structure for automation

### File Organization Rules
- **Documentation**: ALL markdown files must be under `docs/` folder (except main README.md in root)
- **Tests**: ALL tests must be under `tests/` folder, organized into 4 layers:
 - `0.core/` - Core functionality tests
 - `1.unit/` - Unit tests
 - `2.integration/` - Integration tests
 - `3.advance/` - Advance tests (v1.0.0+)
- **Examples**: ALL examples must be under `examples/` folder
- **Source code**: ALL source code must be under `src/` folder
- **Import paths**: Always use `from exonware.library_name import <anything>` for library imports
- **No backward compatibility aliases** - As long as we are in phase 0.x, then never create backward compatibility aliases unless explicitly confirmed (Again, this is only valid in v <1 = NOT STABLE PHASES)

---

## Code Quality Standards

### Import Management

#### **Auto-Install Import Hook (xwsystem [lazy])**

The xwsystem import hook enables deterministic dependency resolution without extra boilerplate:

```python
# Install with [lazy] extra
pip install xwsystem[lazy]

# Then just use normal Python imports!
import fastavro  # Installed on demand when missing
import protobuf  # Installed on demand when missing
import pandas    # Installed on demand when missing

# No try/except guards or HAS_* flags are required
# The import hook performs the installation once and caches the result
```

**Key Engineering Properties:**
- Deterministic: A single line in `__init__.py` enables the behaviour for the package
- Transparent: Successful imports execute at native speed with no additional branching
- Maintainable: Code paths remain free of defensive import guardrails or sentinel flags
- Observable: Install activity is logged once, allowing follow-up actions if needed

#### **Import Rules**
- **Explicit imports only** - No wildcard or fallback imports
- **Complete dependencies** - All formats and their dependencies must be included
- **NO TRY/EXCEPT FOR IMPORTS** - **CRITICAL: Never use try/except blocks for imports. With [lazy] extra, the import hook handles missing packages automatically. Without [lazy], all dependencies must be explicitly declared in requirements. This prevents hidden runtime errors and ensures clean, maintainable code.**
- **NO HAS_* FLAGS** - Don't create `HAS_LIBRARY` flags to check if packages are available. The import hook makes this unnecessary.
- **NO CONDITIONAL IMPORTS** - Import libraries directly. The hook handles missing packages automatically if [lazy] is installed.

#### **3 Installation Modes (MANDATORY for ALL packages)**

All eXonware packages (xwsystem, xwnode, xwdata, xwschema, xwaction, xwentity) and any package using lazy installation **MUST** support 3 installation modes:

**pyproject.toml Structure:**
```toml
[project]
dependencies = [
 "exonware-xwsystem>=0.0.1", # Core dependency
]

[project.optional-dependencies]
lazy = [
 "exonware-xwsystem[lazy]>=0.0.1", # Lazy mode
]

full = [
 "pandas>=2.0.0",
 "numpy>=1.24.0",
 # ... all optional dependencies
]
```

**Installation Modes:**
1. **LITE (default):** `pip install package` - Core dependencies only, minimal footprint
2. **LAZY:** `pip install package[lazy]` - Auto-install on demand, recommended for development
3. **FULL:** `pip install package[full]` - All dependencies pre-installed, recommended for use

**__init__.py Configuration (MANDATORY):**
```python
# Around line 84 in __init__.py
from exonware.xwsystem.utils.lazy_discovery import config_package_lazy_install_enabled
config_package_lazy_install_enabled("package_name") # Auto-detect from installation
```

**Import Pattern (MANDATORY):**
```python
# Standard imports - NO try/except!
import pandas
import numpy
import scikit-learn

# NOT this:
# try:
# import pandas
# except ImportError:
# pandas = None
```

### Code Structure
- **Separation of concerns** - Enforce clear separation of concerns with dedicated modules
- **Design patterns** - Implement software design patterns: strategy-based, handler-based, engine-based
- **Dynamic handlers** - Use dynamic handlers for flexible system behavior
- **Abstract-first approach** - Move as much as possible to abstract classes for better extensibility
- **No separate core.py** - Don't create separate core.py files since __init__.py already properly imports from actual module structure
- **No examples.py in src/** - Examples should not be in the src/ directory structure

### Module Organization

**REQUIRED files at package root (`src/exonware/library_name/`):**
1. **`__init__.py`** - Package initialization and exports
2. **`contracts.py`** - All enums and interfaces (MANDATORY: Never use "protocols.py")
3. **`errors.py`** - All library-specific error classes (file, not folder)
4. **`base.py`** - All abstract classes and base implementations
5. **`facade.py`** - Facade pattern implementation (main public API)
6. **`defs.py`** - Type definitions, constants, enums
7. **`config.py`** - Configuration classes and settings
8. **`version.py`** - Version information (__version__, __author__, etc.)

**OPTIONAL files for sub-modules (`src/exonware/library_name/module_name/`):**
- **`contracts.py`** - Module-specific enums and interfaces (if needed)
- **`errors.py`** - Module-specific error classes (if needed)
- **`base.py`** - Module-specific abstract classes (if needed)
- **`facade.py`** - Module-specific facade (if needed)
- **`defs.py`** - Module-specific definitions (if needed)
- **`config.py`** - Module-specific configuration (if needed)

**File hierarchy rules:**
- Package root files are REQUIRED (all 8 files must exist)
- Sub-module files are OPTIONAL (create only when needed)
- Never mix module-specific and package-level code in the same file
- Each file serves a specific purpose following separation of concerns

### Naming Conventions
- **File naming**: snake_case (e.g., `data_handler.py`, `schema_validator.py`)
- **Class naming**: CapWord (e.g., `DataHandler`, `SchemaValidator`)
- **Library naming**: lowercase (e.g., `xwsystem`, `xnode`, `xdata`, `xschema`, `xaction`, `xentity`)
- **Interface files**: **MANDATORY: Always use `contracts.py` - NEVER use `protocols.py`** (e.g., `contracts.py` for all enums and interfaces)
- **Interfaces**: `IClass` (e.g., `INode`, `IEdge`, `IDataHandler`)
- **Abstract classes**: `AClass` (e.g., `ANode`, `AEdge`, `ABaseHandler`) - **MANDATORY: All abstract classes in base.py files MUST start with 'A' and extend interface class: AClass(IClass)**
- **Key classes**: `XWClass` (e.g., `XWNode`, `XWEdge`, `XWDataHandler`) - Rich design patterns supporting strategy-based, handler-based, engine-based features; `XW` prefix denotes canonical eXonware implementations exposed by facades
- **Handler classes**: Uppercase acronyms ending with `DataHandler` or `SchemaHandler` (e.g., `JSONDataHandler`, `CAPSBaseHandler`, `CAPSDataHandler`)

### Library vs Class Naming Rules
- **Library references**: Always use lowercase (e.g., `xwsystem`, `xnode`, `xdata`)
- **Class references**: Use CapWord with `XW` prefix for canonical classes (e.g., `XWNode`, `XWData`, `XWSchema`)
- **Import statements**: Use library names (lowercase) for imports
- **Class instantiation**: Use class names (CapWord) for creating objects

### Design Patterns

#### **Structural Patterns**
- **Facade pattern** - Mandatory outside xwsystem library for all future libs (xnode, xdata, xschema, xaction, xentity). **Note: xwsystem is an exception** - it does not require a facade pattern as it serves as the foundation library
- **Adapter pattern** - For integrating incompatible interfaces and legacy systems
- **Decorator pattern** - For adding functionality to objects without altering their structure
- **Composite pattern** - For treating individual objects and compositions uniformly
- **Proxy pattern** - For controlling access to objects and adding lazy initialization
- **Bridge pattern** - For separating abstraction from implementation

#### **Creational Patterns**
- **Factory pattern** - For creating objects without specifying their exact classes
- **Abstract Factory pattern** - For creating families of related objects
- **Builder pattern** - For constructing complex objects step by step
- **Singleton pattern** - For ensuring only one instance exists (use sparingly)
- **Prototype pattern** - For creating objects by cloning existing instances
- **Object Pool pattern** - For reusing expensive-to-create objects
- **Lazy Initialization pattern** - Initialize objects only when first accessed for optimal performance
- **Lazy Loading pattern** - Load data only when needed to reduce memory usage
- **Virtual Proxy pattern** - Create placeholder objects that load actual data on demand
- **Lazy evaluation pattern** - Defer computation until results are actually needed

#### **Behavioral Patterns**
- **Strategy pattern** - For interchangeable algorithms and behaviors
- **Observer pattern** - For event-driven architectures and change notifications
- **Command pattern** - For encapsulating requests as objects
- **State pattern** - For managing object behavior based on internal state
- **Template Method pattern** - For defining algorithm skeletons with customizable steps
- **Chain of Responsibility pattern** - For passing requests along a chain of handlers
- **Mediator pattern** - For reducing coupling between communicating objects
- **Memento pattern** - For capturing and restoring object state
- **Visitor pattern** - For adding operations to object structures without modifying them
- **Iterator pattern** - For traversing collections without exposing internal structure

#### **Concurrency Patterns**
- **Producer-Consumer pattern** - For decoupling data production and consumption
- **Thread Pool pattern** - For managing thread lifecycle and resource usage
- **Actor pattern** - For message-passing concurrency and isolation
- **Lock pattern** - For synchronizing access to shared resources
- **Semaphore pattern** - For controlling access to limited resources
- **Barrier pattern** - For synchronizing multiple threads at specific points

#### **Architectural Patterns**
- **MVC (Model-View-Controller)** - For separating data, presentation, and control logic
- **MVP (Model-View-Presenter)** - For improved testability and separation of concerns
- **MVVM (Model-View-ViewModel)** - For data binding and reactive programming
- **Repository pattern** - For abstracting data access logic
- **Unit of Work pattern** - For managing transactions and data consistency
- **CQRS (Command Query Responsibility Segregation)** - For separating read and write operations
- **Event Sourcing pattern** - For storing state changes as a sequence of events
- **Microservices pattern** - For decomposing applications into independent services
- **API Gateway pattern** - For managing and routing client requests
- **Circuit Breaker pattern** - For preventing cascading failures in distributed systems

#### **Domain-Specific Patterns**
- **Handler pattern** - For dynamic request processing and routing
- **Engine pattern** - For complex processing workflows and pipelines
- **Plugin pattern** - For extensible architectures and modular functionality
- **Registry pattern** - For managing and locating services and components
- **Specification pattern** - For encapsulating business rules and criteria
- **Value Object pattern** - For representing immutable domain concepts
- **Aggregate pattern** - For maintaining data consistency in domain models
- **Factory Method pattern** - For creating domain objects with complex initialization

---

## Testing Strategy

**📖 For detailed testing implementation, see [GUIDE_TEST.md](GUIDE_TEST.md)**

### Why Hierarchical Runners and pytest?

**Runners provide consistency and scalability:**
- **Hierarchical architecture** - Main runner orchestrates layer runners; unit runner orchestrates module runners
- **Single entry point** - `tests/runner.py` is the main orchestrator for all test execution
- **Layer runners** - Each test layer (0.core, 1.unit, 2.integration, 3.advance) has its own runner
- **Module runners** - Unit test modules have individual runners for granular control
- **Connected execution** - Runners call sub-runners for comprehensive testing

**pytest enables advanced features:**
- Fixtures, parametrization, and plugins for complex testing scenarios
- Markers for test categorization and selective execution
- Coverage reporting and JUnit XML for CI/CD integration

**Why this matters:**
- **Separation of concerns** - Runners handle orchestration while pytest handles execution details
- **Maintainability** - Centralized test configuration and reporting
- **CI/CD integration** - Standardized test execution for automated pipelines
- **Scalability** - Easy to add new test categories and modules
- **Fast feedback** - Run specific layers or modules independently

### Test Organization (4 Layers)

**Numbered directory structure for clear hierarchy:**

```
tests/
+- runner.py # Main orchestrator
+- 0.core/ # Layer 0: Core tests (80/20 rule)
� +- runner.py # Core test runner
� +- data/ # Test data
� +- test_core_*.py # Core functionality tests
�
+- 1.unit/ # Layer 1: Unit tests
� +- runner.py # Unit test orchestrator
� +- module1_tests/ # Mirrors src structure
� � +- runner.py # Module test runner
� � +- test_*.py
� +- module2_tests/
� +- runner.py
� +- test_*.py
�
+- 2.integration/ # Layer 2: Integration tests
� +- runner.py # Integration test runner
� +- scenarios/ # Real-world scenarios
� +- test_end_to_end.py
�
+- 3.advance/ # Layer 3: Advance tests (OPTIONAL until v1.0.0)
 +- runner.py # Advance test runner
 +- test_security.py # Priority #1
 +- test_usability.py # Priority #2
 +- test_maintainability.py # Priority #3
 +- test_performance.py # Priority #4
 +- test_extensibility.py # Priority #5
```

**Layer purposes:**
- **0.core** - 20% tests for 80% value, fast execution (< 30s)
- **1.unit** - Fine-grained per-module tests, mirrors source structure
- **2.integration** - Cross-module scenarios, real-world flows
- **3.advance** - Excellence validation (v1.0.0+)

**Test focus:**
- Only care about tests under packages directory for targeted validation
- Mirror source structure in unit tests
- Organize by scenarios in integration tests
- Validate excellence in advance tests

### Testing Standards

**Comprehensive coverage across 4 layers:**
- **Core tests** - Critical path coverage, fast failure detection
- **Unit tests** - Individual component validation, = 80% coverage
- **Integration tests** - Cross-module validation, real wiring
- **Advance tests** - Excellence against 5 priorities (v1.0.0+)

**Quality requirements:**
- **No rigged tests** - Avoid rigging tests to pass; fix problems rather than skip them for use reliability
- ** testing** - Build quality with long-term, clean, extensible, maintainable code
- **100% pass requirement** - All tests must pass for release
- **Hierarchical execution** - Main runner calls all sub-runners, all must succeed

**Advance tests (v1.0.0+) validate eXonware's 5 core priorities:**
1. **Security** - OWASP Top 10, defense-in-depth, input validation
2. **Usability** - API intuitiveness, error messages, documentation
3. **Maintainability** - Code quality, design patterns, refactorability
4. **Performance** - Benchmarks, memory usage, scalability
5. **Extensibility** - Plugin support, hooks, customization

**✅ Testing Quality Standards:**
- **100% test pass rate required** - No exceptions
- **Stop on first failure** - Use `-x` or `--maxfail=1` for fast feedback
- **Never hide warnings** - Fix them, don't suppress with `--disable-warnings`
- **Never hide failures** - Don't use `--maxfail=10` or other continuation flags
- **Root cause fixing only** - No workarounds, no rigged tests, no removed features

---

## Documentation Standards

### Documentation Structure
- **Main README.md** - Only one in library root directory
- **Detailed docs** - All detailed documentation in `docs/` folder
- **AI-friendly guides** - Include AI-friendly documentation for better AI assistance

### Documentation Content
- **Killer one-sentence overview** - Start with compelling one-sentence description
- **Comprehensive feature arsenal** - Include detailed sections covering all capabilities
- **Usage examples** - Provide clear, practical examples
- **API documentation** - Complete API reference with examples

---

## Version Management

### eXonware Versioning Philosophy
The eXonware ecosystem follows a structured 5-phase development approach with strict versioning rules:

#### **Version 0.x: Complete Ecosystem Development**
- **Why Version 0 stays 0**: First number remains 0 until ready
- **Complete ecosystem requirement**: All libraries (xwsystem, xnode, xdata, xschema, xaction, xentity) must be fully developed
- **Backend and frontend completion**: All backend systems and frontend applications must be implemented
- **Real-world validation**: At least 3 full-fledged applications must be completed
- **Design pattern perfection**: Extensive usage refines software patterns and architecture
- **Integration testing**: Validate inter-library compatibility and performance

#### **Version 1.x: Ready**
- **Why Version 1 is ready**: Complete ecosystem with proven applications
- **Enterprise deployment**: All components validated in use environments
- **Stability and hardening**: quality across all libraries
- **Comprehensive documentation**: Complete API documentation and developer guides

#### **Version 2.x: Draft Mars Standard**
- **Why Version 2 is draft Mars Standard**: Proven stability enables standardization
- **Interoperability focus**: Cross-platform compatibility and standard API definitions
- **Performance standardization**: Benchmarks and compliance across platforms
- **Security framework alignment**: Consistent security patterns across all libraries

#### **Version 3.x: RUST Core & Multi-Language**
- **Why Version 3 moves to RUST**: Performance optimization with memory safety
- **Multi-language support**: Available in RUST, Python, TypeScript, and Go
- **Facade pattern implementation**: Language-specific facades for all libraries
- **Enterprise scalability**: RUST core enables enterprise-grade performance

#### **Version 4.x: Mars Standard Implementation**
- **Why Version 4 is Mars Standard**: Complete ecosystem maturity with multi-language support
- **Full compliance**: Mars Standard certification across all platforms
- **Global distribution**: Enterprise deployment frameworks for all supported languages
- **Industry adoption**: Partnership frameworks and certification programs

### Version Control Rules
- **Exact version preservation** - Never change versions automatically (like from 0.0.1 to 0.0.1.post1)
- **User-specified versions** - Always keep versions exactly as user specifies
- **No automatic bumps** - Never bump or modify version numbers unless explicitly requested
- **Version 0.0.1 stays 0.0.1** - User wants version 0.0.1 to stay as 0.0.1
- **No shortcuts to v1.0** - Version 0 must complete entire ecosystem before Version 1

### Git Configuration
- **Main branch** - Always use 'main' as the default main branch in Git
- **Consistent Python versions** - Use the same Python version everywhere (avoid mixing versions like 3.8 and 3.9)

### Development Phase Guidelines
- **Version 0.x development**: Focus on completing all libraries and applications
- **No releases**: Version 0.x is experimental and not ready
- **Complete ecosystem first**: All libraries must be developed before moving to Version 1
- **Real-world validation**: At least 3 full applications must be in use
- **Quality over speed**: No phase skipping - each version builds upon the previous

---

## Import & Dependency Management

### Lazy Installation Integration (MANDATORY for ALL eXonware Packages)

All eXonware packages (xwsystem, xwnode, xwdata, xwschema, xwaction, xwentity) **MUST** support lazy installation with 3 installation modes.

#### **Complete Integration Checklist**

**1. pyproject.toml Configuration (MANDATORY):**
```toml
[project]
name = "exonware-yourpackage"
version = "0.0.1"
dependencies = [
 "exonware-xwsystem>=0.0.1", # Core dependency for lazy system
]

[project.optional-dependencies]
# LAZY MODE - Auto-install dependencies on demand (development)
lazy = [
 "exonware-xwsystem[lazy]>=0.0.1",
]

# FULL MODE - Pre-install all dependencies
full = [
 "pandas>=2.0.0",
 "numpy>=1.24.0",
 # ALL optional dependencies here
]
```

**2. __init__.py Setup (MANDATORY - Line ~84):**
```python
# yourpackage/__init__.py
"""
#exonware/yourpackage/src/exonware/yourpackage/__init__.py

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
"""

# LAZY INSTALLATION - Simple One-Line Configuration
from exonware.xwsystem.utils.lazy_discovery import config_package_lazy_install_enabled
config_package_lazy_install_enabled("yourpackage") # Auto-detect [lazy] extra

# IMPORTS - Standard Python Imports (NO try/except!)
import pandas
import numpy
# ... all dependencies
```

**3. Module Imports (MANDATORY - NO Defensive Code):**
```python
# yourpackage/processor.py
import pandas as pd
import numpy as np

# NOT this:
# try:
# import pandas as pd
# except ImportError:
# pd = None
```

**4. README.md Documentation (MANDATORY):**
```markdown
## Installation

### Lite (Default) - Core Only
pip install exonware-yourpackage

### Lazy (Recommended for Development)
pip install exonware-yourpackage[lazy]
Auto-installs missing dependencies on demand

### Full (Recommended)
pip install exonware-yourpackage[full]
All dependencies pre-installed
```

**5. Testing (MANDATORY):**
- Test lite mode: `pip install package`
- Test lazy mode: `pip install package[lazy]`
- Test full mode: `pip install package[full]`
- Verify import hook activation
- Verify auto-installation works

---

### Library Strategy
- **Why minimize imports** - Reduces dependency hell, simplifies deployment, and improves security by reducing attack surface
- **One library import rationale** - Instead of importing 10+ libraries, projects import ONE: xwsystem to achieve dependency consolidation and simplified maintenance
- **Why libraries** - Use libraries instead of custom implementations to leverage battle-tested solutions and reduce development time
- **External library criteria** (Why these criteria matter):
 - Check if library exists - Ensures reliability and community support
 - Use the most established, well-maintained library - Reduces risk of abandonment and security vulnerabilities
 - Wrap it with xwsystem security/validation - Adds enterprise-grade security and validation layers
 - Never rewrite what already exists - Avoids reinventing proven solutions and focuses on value-added features
 - Keep implementations short and delegate to experts - Maintains code simplicity while leveraging specialized expertise

### Package Structure
- **Package naming**: `exonware-libraryname` (pip install exonware-libraryname)
- **Main import path**: `import exonware.libraryname` with convenience alias `import libraryname`
- **Complete replacement**: All xlib references must be completely replaced with exonware throughout codebase, tests, and documentation
- **Dual structure**: Include both `src/exonware/libraryname/` (main package) and `src/libraryname.py` (convenience alias)

---

## eXonware Ecosystem Architecture

### Core Library Dependencies
- **xwsystem** (foundation, core-lib & facade-lib) - Core serialization, security, threading, HTTP client
- **xnode** (core-lib & facade-lib) - Node structures, edge types, xQuery support, lazy initialization
- **xwdata** (core-lib & facade-lib) - Uses xnode, 50+ formats, anything-to-anything conversion
- **xwschema** (core-lib & facade-lib) - Mix of xdata + OpenAPI, schema validation, format conversion
- **xaction** (core-lib & facade-lib) - Function decoration, security, async, scheduling, automation
- **xentity** (core-lib & facade-lib) - Combines xschema + xaction + xdata in unified object

### Application Layer Libraries
- **xauth** (lib, backend API, frontend app) - Extends xentity, implements best practices
- **xstorage** (lib, backend API, frontend app) - xentity storage like Firestore, extensible
- **xbase** (lib, backend API, frontend app) - Firebase/Supabase alternative, easy migration
- **xui** (frontend components lib) - TypeScript UI components

### Library Relationships
- **xnode** ? **xdata** ? **xschema** ? **xaction** ? **xentity**
- **xentity** ? **xauth**, **xstorage**, **xbase**
- **xui** (independent frontend components)

### Ecosystem Design Principles
- **Lazy initialization** - All libraries implement lazy loading for optimal performance
- **Async-first** - All I/O operations are async by default
- **Format-agnostic** - Libraries work with any data format through xdata
- **Extensible architecture** - Easy to add new formats, schemas, and functionality
- **Unified API** - Consistent interface patterns across all libraries

### Core-Facade Architecture
- **Core-lib (Rust future)**: High-performance, low-level implementation with full functionality
- **Facade-lib (Multi-language)**: Simplified API wrapping core-lib complexity
- **Current Phase (0.x)**: Both core and facade implemented in Python
- **Future Phase (3.x)**: Core migrated to Rust, facades available in Python/Rust/TS/Go
- **Language-specific facades**: Each language gets optimized facade implementation
- **Shared core**: Single Rust core serves all language facades
- **Performance benefit**: Rust core provides maximum performance, facades provide language-specific ergonomics

---

## Project Categories & Standards

### Core Libraries (lib=core-lib & facade-lib)
- **xnode, xdata, xschema, xaction, xentity**
- Must implement both core functionality and facade pattern
- **Core-lib**: Low-level implementation with full functionality (currently Python, future Rust)
- **Facade-lib**: Simplified API wrapping core-lib complexity (Python/Rust/TS/Go)
- **Current Phase**: Both core and facade in Python
- **Future Phase**: Core in Rust, facades in multiple languages
- **Requirements**: Lazy initialization, async-first, format-agnostic design

### Standalone Libraries (core-lib)
- **xwsystem** (foundation library)
- Core functionality without facade requirement
- Can be used independently by other libraries
- **Requirements**: quality, comprehensive testing

### Application Libraries (facade-lib)
- **xauth, xstorage, xbase**
- Focus on facade pattern for easy integration
- Built on top of core libraries
- **Requirements**: Simple API, comprehensive documentation

### Backend Web APIs
- **xauth API, xstorage API, xbase API**
- RESTful APIs with OpenAPI documentation
- Async-first implementation
- Comprehensive error handling and logging
- **Requirements**: Security-first, performance-optimized, scalable

### Frontend Components Library
- **xui** - TypeScript UI components
- Framework-agnostic components
- Comprehensive component documentation
- Accessibility compliance
- **Requirements**: Modern UI/UX, responsive design, accessibility

### Frontend Applications
- **xauth app, xstorage app, xbase app**
- Complete frontend applications
- Modern UI/UX with responsive design
- Integration with backend APIs
- **Requirements**: User-friendly, performant, secure

---

## Security & Performance

### Security Standards
- **OWASP Top 10 compliance** - Implement security based on OWASP Top 10 guidelines
- **Defense-in-depth** - Security should be built-in with defense-in-depth approach
- **Input validation** - All external input must be sanitized and validated
- **Path validation** - Include proper path validation and security checks
- **Cryptographic operations** - Use established cryptographic libraries (cryptography for Python, etc.)
- **Static analysis** - Use language-specific security tools (bandit for Python, etc.)

### Performance Optimization
- **Async-first design** - Implement async/await patterns for all I/O operations
- **Concurrent execution** - Use async patterns for parallel processing and non-blocking operations
- **Profiling requirements** - Profile critical code paths using language-specific tools
- **Performance budgets** - Set performance budgets for key API endpoints
- **Benchmarking** - Use tools like pytest-benchmark for performance measurement
- **Memory management** - Automatic memory leak prevention
- **Circuit breakers** - Implement circuit breakers for resilience
- **Monitoring** - Ready monitoring and performance tracking

### Async & Concurrency Standards
- **Async-first architecture** - Design all libraries with async support as the primary interface
- **Language-specific async patterns**:
 - **Python**: async/await, asyncio, async generators, async context managers
 - **Rust**: async/await, tokio, futures, async traits
 - **Go**: goroutines, channels, context, sync primitives
 - **TypeScript**: async/await, Promises, async iterators, async generators
- **Non-blocking I/O** - All file, network, and database operations must be async
- **Concurrent processing** - Use async patterns for parallel execution and batch operations
- **Async context management** - Proper resource cleanup in async environments
- **Error handling in async** - Comprehensive async exception handling and propagation
- **Lazy async initialization** - Combine lazy loading with async operations for optimal performance

---

## Code Review & CI/CD Standards

### Pull Request Requirements
- **All changes via Pull Request** - No direct commits to main branch
- **Automated CI checks** - PR must pass all automated checks: tests, lint, type-check, docs build
- **Code review approval** - At least one code review approval required for merge
- **Branch naming conventions** - Use feature/bugfix branch naming conventions
- **PR templates** - Use PR templates to ensure checklist compliance (tests, docs, changelog)

### Language-Specific Quality Tools
- **Python**: flake8, black, isort, mypy, pytest
- **Rust**: cargo fmt, clippy, cargo test
- **Go**: gofmt, go vet, go test
- **TypeScript**: eslint, prettier, tsc, jest

### CI/CD Requirements
- **All merges to main** require passing tests, type-check, lint, and docs build
- **PR template checklist**: tests, docs, changelog, issue reference
- **Security review** for all third-party dependencies
- **Dependabot integration** or equivalent for dependency updates

### Security in CI/CD
- **Static analysis** - Run language-specific security tools in CI
- **Dependency scanning** - Scan for known vulnerabilities
- **Secret scanning** - Prevent accidental secret commits
- **License compliance** - Ensure all dependencies have compatible licenses

---

## AI Development Guidelines

### AI Standards
- **Follow ALL guidelines** - This entire document serves as the comprehensive guide for ALL work
- **Complete adherence** - Every section, rule, and principle must be followed for any output
- **Not just code** - Guidelines apply to documentation, testing, project structure, versioning, and all deliverables
- **Quality assurance** - Use this document as the quality standard for all AI-generated content
- **No bias manipulation** - Never use inline comments or marketing language to influence AI evaluation or bias judgement (e.g., "production-grade", "enterprise-ready", "bulletproof", "revolutionary")

### AI-Assisted Development
- **Review real code** - Always review real code before explaining; don't make assumptions
- **Direct IDE edits** - Make changes directly in IDE via file edits rather than inline snippets
- **Complete implementations** - Include all necessary imports and fix problems
- **Avoid repetition** - Don't repeat information already in function signatures (parameter types, optional annotations)
- **Follow naming conventions** - Use lowercase for libraries (xwsystem, xnode) and CapWord for canonical classes with the `XW` prefix (XWNode, XWData)
- **Apply design patterns** - Implement facade, strategy, handler, and engine patterns as specified
- **Enforce separation of concerns** - Organize code with contracts.p, errors.py, base.py structure

### AI Code Generation
- **Comprehensive solutions** - Provide complete, working solutions following all guidelines
- **Error handling** - Include proper error handling and edge cases
- **Documentation** - Generate comprehensive documentation alongside code
- **Testing** - Include test cases and validation using pytest and runners
- **Version compliance** - Ensure all work aligns with versioning philosophy (0.x until complete ecosystem)
- ** quality** - Build long-term, clean, extensible, maintainable code
- **Lazy installation integration** - **MANDATORY: ALL new packages MUST include lazy installation support with 3 installation modes (lite/lazy/full)**
- **Objective code comments** - Write factual, technical comments only; avoid marketing language or bias-inducing terms in code comments

### AI Documentation Standards
- **Follow documentation guidelines** - Place all docs in docs/ folder, use killer one-sentence overviews
- **Include WHY explanations** - Don't just explain WHAT, explain WHY decisions were made
- **AI-friendly format** - Structure content for both human and AI consumption
- **Complete coverage** - Document all aspects following eXonware standards
- **Objective language** - Use factual, technical language; avoid marketing hyperbole or bias-inducing adjectives (e.g., "production-grade", "bulletproof", "revolutionary", "enterprise-ready")

### AI Project Management
- **Follow project structure** - Use standard directory structure for all libraries
- **Apply versioning rules** - Respect version 0.x requirements and no automatic version changes
- **Implement testing strategy** - Use runners and pytest for all testing implementations
- **Follow import management** - Use explicit imports, complete dependencies, libraries

### AI Quality Assurance
- **Comprehensive review** - Check all output against every guideline in this document
- **Readiness** - Ensure all work meets standards
- **Consistency enforcement** - Maintain consistency across all deliverables
- **Guideline compliance** - Verify adherence to all rules and principles
- **Security validation** - Ensure all code follows OWASP Top 10 guidelines
- **Performance assessment** - Validate performance budgets and benchmarking requirements
- **Date accuracy** - Always use accurate, current dates in documentation, headers, and version information

### AI Date Accuracy Standards
- **Why date accuracy matters** - Accurate dates ensure proper versioning, documentation currency, and legal compliance
- **Always use current dates** - Never use placeholder or outdated dates in any work
- **Standardized date formats** - Use consistent date formatting across all eXonware projects:
 - **Documentation format**: `DD-MMM-YYYY` (e.g., "07-Sep-2025")
 - **File names format**: `YYYYMMDD` (e.g., "20250907")
 - **Numbers format**: `###,###.00` (e.g., "1,234.56")
 - **Currency format**: `###,###.00 CUR` (e.g., "1,234.56 USD") - Use 3-letter UN currency codes
- **Cross-platform date commands** - Use appropriate date commands for different operating systems:
 - **Windows PowerShell**: `Get-Date -Format "dd-MMM-yyyy"` (e.g., "07-Sep-2025")
 - **Windows CMD**: `echo %date%` or `wmic os get localdatetime`
 - **Linux/macOS**: `date "+%d-%b-%Y"` (e.g., "07-Sep-2025")
 - **Python**: `datetime.now().strftime("%d-%b-%Y")`
 - **JavaScript/Node.js**: `new Date().toLocaleDateString('en-GB', { year: 'numeric', month: 'short', day: '2-digit' }).replace(' ', '-')`
- **Documentation headers** - Always include accurate generation dates in file headers and documentation using DD-MMM-YYYY format
- **Version information** - Ensure version dates match actual development timeline
- **Legal compliance** - Accurate dates are crucial for licensing, copyright, and legal documentation

### AI Base.py File Management Standards
- **CRITICAL: Never overwrite existing base.py files** - Always read and understand the existing base.py content before making any changes
- **Why this matters** - Base.py files often contain sophisticated, implementations that are imported and used throughout the system; overwriting them causes catastrophic failures
- **Always check existing imports** - Before modifying any base.py file, search the codebase for imports from that file to understand what classes and interfaces are being used
- **Base.py files must extend contracts.py interfaces** - All abstract classes in base.py must implement or extend the interfaces defined in the corresponding contracts.py file
- **Interface inheritance is mandatory** - Abstract classes in base.py should inherit from interfaces in contracts.py to ensure proper contract compliance
- **Preserve existing functionality** - When updating base.py files, preserve all existing classes, methods, and functionality; only add new features or fix bugs
- **Read before writing** - Always use read_file to examine the current content of base.py before making any modifications
- **Understand dependencies** - Use grep or codebase search to find all files that import from base.py to understand the impact of changes
- **Incremental changes only** - Make small, incremental changes to base.py files rather than complete rewrites
- **Test existing functionality** - When modifying base.py files, ensure all existing functionality still works correctly

#### **Abstract Classes: Base.py File Modification Process**
```
MANDATORY PROCESS for modifying any base.py file:

1. READ FIRST: Always read the existing base.py file completely
2. SEARCH DEPENDENCIES: Find all files that import from this base.py
3. UNDERSTAND INTERFACES: Check the corresponding contracts.py for required interfaces
4. PRESERVE EXISTING: Never remove or modify existing classes without explicit permission
5. EXTEND INTERFACES: Ensure new abstract classes extend appropriate interfaces from contracts.py
6. INCREMENTAL CHANGES: Make only necessary additions or fixes
7. VERIFY IMPORTS: Ensure all existing imports continue to work
8. TEST FUNCTIONALITY: Verify that existing functionality is not broken

NEVER:
- Overwrite an entire base.py file without reading it first
- Remove existing classes or methods
- Ignore interface inheritance requirements
- Make assumptions about what should be in base.py
- Create base.py files that don't extend contracts.py interfaces
```

#### **Interface Classes: Interface-Base Relationship Requirements**
- **Contracts define interfaces** - All interfaces (IClass) must be defined in contracts.py
- **Base implements interfaces** - All abstract classes (AClass) in base.py must extend interfaces from contracts.py
- **MANDATORY AClass naming** - **ALL abstract classes in base.py files MUST start with 'A' (e.g., AHandler, AValidator, AManager)**
- **Consistent naming** - Interface IExample should have corresponding abstract class AExample in base.py
- **Complete implementation** - Base.py abstract classes should provide skeletal implementations of interface methods
- **Documentation alignment** - Base.py classes should include comprehensive documentation explaining their relationship to interfaces

### AI Prompt Examples

#### **Code Generation Prompts**

**Library Creation:**
```
Create a new eXonware library called 'xentity' following all DEV_GUIDELINES.md standards:

CORE REQUIREMENTS:
- Use proper naming: library name 'xentity', main class 'XEntity'
- Implement MANDATORY facade pattern for simplified API
- Create contracts.py, errors.py, base.py module structure
- Include comprehensive testing with pytest and runners
- Follow version 0.x development phase requirements
- Generate complete documentation in docs/ folder
- **MANDATORY: Include lazy installation support with 3 modes (lite/lazy/full)**

LAZY INSTALLATION (MANDATORY):
- pyproject.toml MUST have: dependencies=["exonware-xwsystem>=0.0.1"]
- pyproject.toml MUST have: lazy=["exonware-xwsystem[lazy]>=0.0.1"]
- pyproject.toml MUST have: full=[...all optional dependencies...]
- __init__.py MUST have (line ~84): config_package_lazy_install_enabled("xentity")
- All imports MUST be standard (NO try/except ImportError)
- NO HAS_* flags anywhere in the code
- README MUST document all 3 installation modes

MAIN CLASSES & INTERFACES:
- IEntity (interface) - Core entity contract
- AEntity (abstract) - Base entity implementation
- XEntity (main class) - Rich extensible entity with design patterns
- EntityManager (handler) - Entity lifecycle management
- EntityValidator (strategy) - Validation strategies
- EntityRepository (facade) - Data access facade

KEY OBJECTIVES & FEATURES:
- Entity lifecycle management (create, read, update, delete)
- Validation and constraint enforcement
- Relationship management between entities
- Persistence abstraction with multiple backends
- Event-driven architecture for entity changes
- Performance optimization with caching strategies

DESIGN PATTERNS TO IMPLEMENT:
- Facade pattern (MANDATORY) - Unified interface to complex entity operations
- Strategy pattern - Interchangeable validation and persistence strategies
- Observer pattern - Event-driven entity change notifications
- Repository pattern - Data access abstraction
- Factory pattern - Entity creation and configuration
- Command pattern - Encapsulate entity operations as commands
- State pattern - Manage entity lifecycle states
- Decorator pattern - Add functionality to entities without modification
- Proxy pattern - Control access to entity operations
- Builder pattern - Construct complex entity configurations
- Specification pattern - Encapsulate entity business rules
- Value Object pattern - Represent immutable entity properties

PRIORITIES FOCUS:
1. Usability - Simple, intuitive entity operations
2. Maintainability - Clean, well-structured entity management
3. Performance - Efficient entity operations and caching
4. Extensibility - Easy to extend with new entity types
5. Security - Secure entity access and validation
```

**Feature Implementation:**
```
Implement a new serialization format handler for the xwsystem library:

CORE REQUIREMENTS:
- Follow naming conventions: JSONDataHandler class
- Use abstract base classes from base.py module
- Include error handling in errors.py file
- Add comprehensive tests in tests/unit/ and tests/integration/
- Update documentation following eXonware standards

DESIGN PATTERNS TO APPLY:
- Strategy pattern - Interchangeable serialization algorithms
- Template method pattern - Common serialization workflow
- Factory pattern - Handler creation and configuration
- Chain of responsibility - Format detection and routing
- Adapter pattern - Integrate with legacy serialization systems
- Decorator pattern - Add compression, encryption, validation layers
- Proxy pattern - Lazy loading and caching of serialized data
- Builder pattern - Construct complex serialization configurations
- Observer pattern - Notify on serialization events
- Command pattern - Encapsulate serialization operations
- Registry pattern - Manage available serialization handlers

PRIORITIES FOCUS:
1. Usability - Simple, consistent serialization API
2. Maintainability - Clean, extensible handler architecture
3. Performance - Efficient serialization with minimal overhead
4. Extensibility - Easy to add new format handlers
5. Security - Safe serialization with validation
```

**Code Review:**
```
Review this code against DEV_GUIDELINES.md standards:

COMPREHENSIVE REVIEW CHECKLIST:
- Check naming conventions (libraries lowercase, classes CapWord)
- Verify design patterns implementation (facade, strategy, handler, engine)
- Ensure separation of concerns (contracts.py, errors.py, base.py)
- Validate testing approach (pytest, runners)
- Confirm documentation standards compliance

DESIGN PATTERNS ASSESSMENT:
- Evaluate pattern suitability for the problem domain
- Check for over-engineering vs. appropriate complexity
- Verify pattern implementation follows best practices
- Assess extensibility and maintainability benefits
- Review structural patterns (Facade, Adapter, Decorator, Composite, Proxy, Bridge)
- Validate creational patterns (Factory, Builder, Singleton, Prototype, Object Pool)
- Check behavioral patterns (Strategy, Observer, Command, State, Template Method)
- Assess concurrency patterns (Producer-Consumer, Thread Pool, Actor, Lock)
- Evaluate architectural patterns (MVC, Repository, CQRS, Event Sourcing, Circuit Breaker)
- Review domain-specific patterns (Handler, Engine, Plugin, Registry, Specification)

PRIORITIES ALIGNMENT:
1. Usability - Is the code easy to use and understand?
2. Maintainability - Is the code clean and well-structured?
3. Performance - Are there any performance concerns?
4. Extensibility - Can the code be easily extended?
5. Security - Are security considerations properly addressed?
```

#### **Documentation Prompts**

**API Documentation:**
```
Generate comprehensive API documentation for the XData class:

CORE REQUIREMENTS:
- Follow eXonware documentation standards
- Include killer one-sentence overview
- Provide clear usage examples
- Explain WHY design decisions were made
- Structure for both human and AI consumption
- Place in docs/ folder following guidelines

DESIGN PATTERNS DOCUMENTATION:
- Document implemented design patterns (facade, strategy, etc.)
- Explain pattern benefits for usability and maintainability
- Show how patterns support extensibility and performance
- Include security considerations in pattern usage

PRIORITIES FOCUS:
1. Usability - Clear, intuitive API documentation
2. Maintainability - Well-structured, comprehensive docs
3. Performance - Document performance characteristics
4. Extensibility - Show how to extend and customize
5. Security - Document security features and considerations
```

**Project Documentation:**
```
Create project documentation for a new eXonware library:
- Follow standard directory structure
- Include WHY explanations for all decisions
- Generate AI-friendly format
- Cover all aspects: setup, usage, testing, deployment
- Ensure comprehensive coverage following eXonware standards
```

#### **Testing Prompts**

**Test Generation:**
```
Generate comprehensive tests for the XWNode class:

CORE REQUIREMENTS:
- Use pytest and runners following DEV_GUIDELINES.md
- Include core/, unit/, and integration/ test categories
- Follow testing standards (no rigged tests, comprehensive coverage)
- Use proper test organization and structure
- Include verify_installation.py test

DESIGN PATTERNS TESTING:
- Test facade pattern implementation and API consistency
- Validate strategy pattern behavior with different strategies
- Test handler pattern with various handler types
- Verify pattern interactions and edge cases

PRIORITIES FOCUS:
1. Usability - Test API ease of use and error handling
2. Maintainability - Test code structure and organization
3. Performance - Include performance benchmarks and tests
4. Extensibility - Test extension points and customization
5. Security - Test security features and vulnerability prevention
```

**Test Review:**
```
Review these tests against DEV_GUIDELINES.md testing standards:
- Verify pytest and runners usage
- Check test organization (core/, unit/, integration/)
- Ensure comprehensive coverage
- Validate no rigged tests
- Confirm testing quality
```

#### **Project Structure Prompts**

**Library Setup:**
```
Set up a new eXonware library following DEV_GUIDELINES.md:

CORE REQUIREMENTS:
- Create standard directory structure
- Implement contracts.py, errors.py, base.py modules
- Set up tests/ with proper organization
- Configure pyproject.toml and requirements.txt
- Generate initial documentation structure

DESIGN PATTERNS FOUNDATION:
- Implement facade pattern as MANDATORY requirement
- Set up strategy pattern infrastructure for extensibility
- Create handler pattern framework for dynamic behavior
- Establish factory pattern for object creation
- Design for observer pattern if event-driven features needed
- Implement repository pattern for data access abstraction
- Set up command pattern for operation encapsulation
- Create builder pattern for complex object construction
- Establish decorator pattern for functionality extension
- Design proxy pattern for access control and lazy loading
- Implement registry pattern for service management
- Set up specification pattern for business rule encapsulation

PRIORITIES FOCUS:
1. Usability - Simple, intuitive library structure
2. Maintainability - Clean, organized codebase
3. Performance - Efficient foundation for high performance
4. Extensibility - Easy to extend and customize
5. Security - Secure by design architecture
```

**Migration:**
```
Migrate existing code to follow DEV_GUIDELINES.md standards:
- Update naming conventions (libraries lowercase, classes CapWord)
- Implement proper module organization (contracts.py, errors.py, base.py)
- Restructure tests following guidelines
- Update documentation to docs/ folder
- Ensure version compliance (0.x development phase)
```

#### **Version Management Prompts**

**Version Planning:**
```
Plan version progression for the eXonware ecosystem:
- Follow versioning philosophy (0.x until complete ecosystem)
- Ensure all libraries (xwsystem, xnode, xdata, xschema, xaction, xentity) are complete
- Validate 3+ full applications requirement
- Plan transition to Version 1.x (ready)
- Maintain exact version preservation rules
```

**Release Management:**
```
Prepare release following DEV_GUIDELINES.md versioning rules:
- Verify complete ecosystem development
- Check all libraries are fully developed
- Validate 3+ applications
- Ensure no automatic version changes
- Follow exact version preservation requirements
```

#### **Quality Assurance Prompts**

**Comprehensive Review:**
```
Perform comprehensive review of this eXonware library:
- Check against ALL DEV_GUIDELINES.md sections
- Verify naming conventions compliance
- Validate design patterns implementation
- Ensure testing standards adherence
- Confirm documentation completeness
- Check version management compliance
```

**Readiness:**
```
Assess readiness following DEV_GUIDELINES.md:
- Verify quality standards
- Check comprehensive test coverage
- Validate security and performance standards
- Ensure complete documentation
- Confirm version compliance
- Assess enterprise readiness
```

#### **Design Pattern Prompts**

**Facade Implementation:**
```
Implement facade pattern for xdata library:

CORE REQUIREMENTS:
- Follow MANDATORY facade pattern requirements
- Create unified interface to complex subsystems
- Hide implementation complexity
- Ensure maintainability and consistency
- Apply to all future libraries (xnode, xschema, xaction, xentity)

DESIGN PATTERNS INTEGRATION:
- Combine facade with strategy pattern for algorithm selection
- Use factory pattern within facade for object creation
- Implement observer pattern for change notifications
- Apply template method pattern for common workflows
- Integrate command pattern for operation encapsulation
- Use decorator pattern to add layers of functionality
- Implement proxy pattern for access control and caching
- Apply builder pattern for complex configuration
- Use adapter pattern for legacy system integration
- Implement registry pattern for service discovery
- Apply specification pattern for business rule validation
- Use state pattern for lifecycle management

PRIORITIES FOCUS:
1. Usability - Simple, unified API hiding complexity
2. Maintainability - Clean facade interface, easy to modify
3. Performance - Efficient facade with minimal overhead
4. Extensibility - Easy to extend facade with new features
5. Security - Secure facade with proper validation
```

**Strategy Pattern:**
```
Implement strategy pattern for serialization handlers:

CORE REQUIREMENTS:
- Create interchangeable algorithms and behaviors
- Use proper naming conventions (JSONDataHandler, XMLDataHandler)
- Follow separation of concerns
- Implement in base.py module with abstract classes
- Ensure extensibility and maintainability

DESIGN PATTERNS INTEGRATION:
- Combine with factory pattern for strategy creation
- Use facade pattern to hide strategy complexity
- Implement chain of responsibility for strategy selection
- Apply template method for common strategy workflow
- Use decorator pattern to add strategy enhancements
- Implement observer pattern for strategy change notifications
- Apply command pattern for strategy operations
- Use registry pattern for strategy management
- Implement proxy pattern for strategy access control
- Apply builder pattern for strategy configuration
- Use adapter pattern for legacy strategy integration
- Implement specification pattern for strategy validation

PRIORITIES FOCUS:
1. Usability - Simple strategy selection and usage
2. Maintainability - Clean strategy interface and implementation
3. Performance - Efficient strategy execution and switching
4. Extensibility - Easy to add new strategies
5. Security - Secure strategy execution with validation
```

#### **Integration Prompts**

**Library Integration:**
```
Integrate xnode with xdata following DEV_GUIDELINES.md:
- Implement delegation pattern (xnode delegates to xdata)
- Ensure format-agnostic design
- Follow naming conventions
- Maintain separation of concerns
- Validate inter-library compatibility
```

**Ecosystem Development:**
```
Develop complete eXonware ecosystem following DEV_GUIDELINES.md:
- Complete all libraries (xwsystem, xnode, xdata, xschema, xaction, xentity)
- Implement backend systems using ecosystem
- Create frontend applications
- Develop 3+ full-fledged applications
- Follow version 0.x development requirements
```

#### **eXonware Ecosystem-Specific Prompts**

**xnode Implementation:**
```
Implement xnode library with Strategy Design Patterns following DEV_GUIDELINES.md:

CORE REQUIREMENTS:
- Implement 28 Node Modes: HASH_MAP, ARRAY_LIST, LSM_TREE, ROARING_BITMAP, UNION_FIND, SEGMENT_TREE, etc.
- Implement 16 Edge Modes: ADJ_LIST, ADJ_MATRIX, R_TREE, TEMPORAL_EDGESET, NEURAL_GRAPH, etc.
- Strategy Manager with lazy materialization and auto mode selection
- A+ Usability Presets: DATA_INTERCHANGE_OPTIMIZED, SOCIAL_GRAPH, ANALYTICS, SEARCH_ENGINE, etc.
- Advanced Traits: WEIGHTED, PROBABILISTIC, SPATIAL, STREAMING, etc.
- Performance monitoring and strategy migration capabilities
- Design for core-facade separation (current Python, future Rust core)

CORE-FACADE ARCHITECTURE:
- Core-lib: Low-level node operations and data structures
- Facade-lib: Simplified API wrapping core complexity
- Current Phase: Both core and facade in Python
- Future Phase: Core in Rust, facades in Python/Rust/TS/Go
- Design for easy separation and migration

ADVANCED FEATURES:
- Strategy Manager with lazy materialization
- Auto mode selection with performance heuristics
- Strategy migration with data preservation
- Performance monitoring and capability checking
- A+ Usability Presets for different use cases
- Advanced traits system for cross-cutting capabilities

PRIORITIES FOCUS:
1. Security - Secure node operations and data access
2. Usability - Simple node manipulation API with presets
3. Maintainability - Clean node structure management
4. Performance - Lazy loading, async operations, and strategy optimization
5. Extensibility - Easy to add new node/edge types and strategies
```

**xdata Implementation:**
```
Implement xdata library with Data Framework following DEV_GUIDELINES.md:

CORE REQUIREMENTS:
- Support 50+ data formats (exceeding xwsystem's 24)
- Implement anything-to-anything conversion
- COW Semantics for optimal memory usage and performance
- Object Pooling with pre-allocated node pools for hot path operations
- Performance Caching with parse/serialize caches and structural hashing
- Security Integration with path validation, file size limits, content validation
- Format Auto-Detection from content, extensions, and MIME types
- Use xnode for data structure management
- Extensible architecture for custom formats
- Async-first implementation with lazy loading
- Design for core-facade separation (current Python, future Rust core)

CORE-FACADE ARCHITECTURE:
- Core-lib: Low-level format conversion and data handling
- Facade-lib: Simplified API wrapping core complexity
- Current Phase: Both core and facade in Python
- Future Phase: Core in Rust, facades in Python/Rust/TS/Go
- Design for easy separation and migration

CONVERSION PATTERNS:
- Strategy pattern for format conversion algorithms
- Factory pattern for format handler creation
- Adapter pattern for legacy format integration
- Lazy initialization for large data sets
- Async processing for batch conversions

ADVANCED FEATURES:
- COW Semantics for memory optimization
- Object Pooling for performance
- Performance Caching with structural hashing
- Security Integration with comprehensive validation
- Format Auto-Detection with intelligent detection
- Fluent API with method chaining

PRIORITIES FOCUS:
1. Security - Secure data conversion and validation
2. Usability - Simple conversion API with fluent design
3. Maintainability - Clean format handler architecture
4. Performance - Lazy loading, async conversion, and caching
5. Extensibility - Easy to add new formats and handlers
```

**xschema Implementation:**
```
Implement xschema library with Schema Validation Engine following DEV_GUIDELINES.md:

CORE REQUIREMENTS:
- Combine xdata with OpenAPI schema validation
- Implement all known schema validation rules
- Support schema format conversion
- Handler Caching with performance-optimized handler management
- Format Auto-Detection with advanced content-based detection
- Structural Hashing for fast equality checks and caching
- Performance Optimization with ultra-fast implementation
- Define detailed input/output schemas
- Unlimited property detail specifications
- Design for core-facade separation (current Python, future Rust core)

CORE-FACADE ARCHITECTURE:
- Core-lib: Low-level schema validation and processing
- Facade-lib: Simplified API wrapping core complexity
- Current Phase: Both core and facade in Python
- Future Phase: Core in Rust, facades in Python/Rust/TS/Go
- Design for easy separation and migration

SCHEMA PATTERNS:
- Strategy pattern for validation algorithms
- Factory pattern for schema handler creation
- Adapter pattern for different schema formats
- Lazy initialization for large schemas
- Async validation for complex schemas

ADVANCED FEATURES:
- Handler Caching for performance optimization
- Format Auto-Detection with intelligent detection
- Structural Hashing for fast operations
- Performance Optimization with backward compatibility
- Intelligent caching system for schema operations

PRIORITIES FOCUS:
1. Security - Secure schema validation and enforcement
2. Usability - Simple schema definition and validation
3. Maintainability - Clean schema architecture
4. Performance - Lazy loading, async validation, and caching
5. Extensibility - Easy to add new schema types and handlers
```

**xaction Implementation:**
```
Implement xaction library with Enterprise Action Framework following DEV_GUIDELINES.md:

CORE REQUIREMENTS:
- Advanced function decoration and management
- OpenAPI 3.1 Compliance with full API documentation generation
- Security Integration with OAuth2, API keys, MFA, rate limiting, audit trails
- Workflow Orchestration with multi-step workflows, monitoring, and rollback
- Pluggable Engines supporting Native, FastAPI, Celery, Prefect
- Contract Validation with xSchema integration
- Cross-cutting Concerns handlers for Validation, Security, Monitoring, Workflow
- Deep specifications for security, async, scheduling, automation
- Native async function support
- Scheduling and automation capabilities
- Design for core-facade separation (current Python, future Rust core)

CORE-FACADE ARCHITECTURE:
- Core-lib: Low-level function execution and management
- Facade-lib: Simplified API wrapping core complexity
- Current Phase: Both core and facade in Python
- Future Phase: Core in Rust, facades in Python/Rust/TS/Go
- Design for easy separation and migration

ACTION PATTERNS:
- Command pattern for function operations
- Strategy pattern for different action types
- Factory pattern for action creation
- Observer pattern for action events
- Lazy initialization for action configurations

ADVANCED FEATURES:
- OpenAPI 3.1 Compliance for API documentation
- Security Integration with comprehensive authentication
- Workflow Orchestration with state management
- Pluggable Engines for different execution environments
- Contract Validation with schema integration
- Cross-cutting Concerns handlers

PRIORITIES FOCUS:
1. Security - Secure function execution and management
2. Usability - Simple function decoration API with OpenAPI compliance
3. Maintainability - Clean action architecture with pluggable engines
4. Performance - Efficient action execution with workflow optimization
5. Extensibility - Easy to add new action types and engines
```

**xentity Implementation:**
```
Implement xentity library with Advanced Entity Management following DEV_GUIDELINES.md:

CORE REQUIREMENTS:
- Unified object combining xschema + xaction + xdata
- Immutable Facade with thread-safe immutable-style API
- Performance Caching with thread-safe LRU cache and hit/miss tracking
- Lazy Loading with optimized entity loading and access patterns
- Performance Monitoring with comprehensive metrics and operation tracking
- Built-in schema validation for all operations
- Complex entity actions with specifications
- Universal persistence to any supported format
- Lazy initialization for optimal performance
- Design for core-facade separation (current Python, future Rust core)

CORE-FACADE ARCHITECTURE:
- Core-lib: Low-level entity operations and data management
- Facade-lib: Simplified API wrapping core complexity
- Current Phase: Both core and facade in Python
- Future Phase: Core in Rust, facades in Python/Rust/TS/Go
- Design for easy separation and migration

ENTITY PATTERNS:
- Facade pattern for unified entity interface
- Strategy pattern for different entity types
- Factory pattern for entity creation
- Repository pattern for entity persistence
- Lazy initialization for entity loading

ADVANCED FEATURES:
- Immutable Facade for thread-safe operations
- Performance Caching with LRU cache and metrics
- Lazy Loading for memory efficiency
- Performance Monitoring with comprehensive tracking
- Thread-safe operations with optimized performance
- Advanced entity state management

PRIORITIES FOCUS:
1. Security - Secure entity operations and data access
2. Usability - Simple entity manipulation API with immutable design
3. Maintainability - Clean entity architecture with performance monitoring
4. Performance - Lazy loading, async operations, and caching
5. Extensibility - Easy to extend entity functionality with advanced features
```

---

## Library-Specific Patterns

### Why Facade Pattern is Mandatory
- **Simplified API** - Provides a unified interface to complex subsystems, reducing cognitive load for developers
- **Abstraction layer** - Hides implementation complexity while exposing essential functionality
- **Maintainability** - Changes to internal implementation don't affect client code
- **Consistency** - Ensures uniform API across all eXonware libraries (xnode, xdata, xschema, xaction, xentity)
- **Exception: xwsystem** - xwsystem is the foundation library and does not require a facade pattern as it serves as the base for other libraries

### xnode Library (core-lib & facade-lib)
- **Why format-agnostic**: Ensures node operations work regardless of underlying data format
- **28 Node Modes**: Comprehensive data structures from basic (HASH_MAP, ARRAY_LIST) to advanced (LSM_TREE, ROARING_BITMAP, UNION_FIND, SEGMENT_TREE)
- **16 Edge Modes**: Graph structures from basic (ADJ_LIST, ADJ_MATRIX) to specialized (R_TREE, TEMPORAL_EDGESET, NEURAL_GRAPH)
- **Strategy Manager**: Lazy materialization, auto mode selection, performance monitoring, strategy migration
- **A+ Usability Presets**: 12+ predefined configurations (DATA_INTERCHANGE_OPTIMIZED, SOCIAL_GRAPH, ANALYTICS, SEARCH_ENGINE, etc.)
- **Advanced Traits**: 12+ cross-cutting capabilities (WEIGHTED, PROBABILISTIC, SPATIAL, STREAMING, etc.)
- **Lazy initialization**: Initialize nodes and edges only when accessed for optimal performance
- **Performance Monitoring**: Operation tracking, migration planning, capability checking
- **Strategy Migration**: Runtime strategy switching with data preservation
- **Delegation pattern rationale**: Leverages xdata's serialization expertise, avoiding code duplication
- **Virtual proxy pattern**: Create placeholder objects that load actual data on demand
- **Async-first design**: All node operations are async for high performance
- **Core-Facade separation**: Core-lib (Rust future) handles low-level operations, facade provides simplified API
- **Multi-language facades**: Python/Rust/TS/Go facades wrapping shared Rust core

### xdata Library (core-lib & facade-lib)
- **Why format-agnostic**: Enables seamless switching between data formats without changing client code
- **50+ formats**: Exceed xwsystem's 24 formats, extensible to custom formats
- **Anything-to-anything conversion**: Universal format conversion capabilities
- **COW Semantics**: Copy-on-write for optimal memory usage and performance
- **Object Pooling**: Pre-allocated node pools for hot path operations and memory efficiency
- **Performance Caching**: Parse/serialize caches with structural hashing for ultra-fast operations
- **Security Integration**: Path validation, file size limits, content validation, untrusted data handling
- **Format Auto-Detection**: Intelligent format detection from content, extensions, and MIME types
- **Uses xnode**: Built on xnode for data structure management
- **Lazy loading**: Load data only when needed to reduce memory usage
- **Method naming rationale**: `to_native/from_native` clearly indicates data transformation direction
- **Config exclusion**: Prevents configuration pollution in serialized data, maintaining data purity
- **Return XData instances**: Enables fluent API design for method chaining and improved developer experience
- **Core-Facade separation**: Core-lib (Rust future) handles format conversion, facade provides unified API
- **Multi-language facades**: Python/Rust/TS/Go facades wrapping shared Rust core

### xschema Library (core-lib & facade-lib)
- **xdata + OpenAPI**: Combines data handling with comprehensive schema validation
- **All schema rules**: Implement all known schema validation rules and constraints
- **Format conversion**: Convert schemas between any supported formats
- **Handler Caching**: Performance-optimized handler management with intelligent caching
- **Format Auto-Detection**: Advanced content-based format detection and validation
- **Structural Hashing**: Fast equality checks and caching for schema operations
- **Performance Optimization**: Ultra-fast implementation with backward compatibility
- **Input/output schemas**: Define detailed input and output schemas with unlimited property details
- **Constraint focus rationale**: Python's type system already handles basic constraints, focus on advanced validation
- **xAction integration**: Enables declarative validation with imperative actions for complex scenarios
- **Core-Facade separation**: Core-lib (Rust future) handles schema validation, facade provides simplified API
- **Multi-language facades**: Python/Rust/TS/Go facades wrapping shared Rust core

### xaction Library (core-lib & facade-lib)
- **Function decoration**: Advanced function decoration and management capabilities
- **Deep specifications**: Comprehensive function specifications for security, async, scheduling, automation
- **Security integration**: Built-in security for functions with comprehensive validation
- **Async support**: Native async function support with proper error handling
- **Scheduling and automation**: Built-in scheduling and automation capabilities
- **Command pattern**: Encapsulate function operations as commands for better management
- **Core-Facade separation**: Core-lib (Rust future) handles function execution, facade provides simplified API
- **Multi-language facades**: Python/Rust/TS/Go facades wrapping shared Rust core

### xentity Library (core-lib & facade-lib) - Advanced Entity Management
- **Unified object**: Combines xschema + xaction + xdata in a single, powerful object
- **Immutable Facade**: Thread-safe immutable-style API with performance optimizations
- **Performance Caching**: Thread-safe LRU cache with hit/miss tracking and metrics
- **Lazy Loading**: Optimized entity loading and access patterns for memory efficiency
- **Performance Monitoring**: Comprehensive metrics and operation tracking
- **Schema checking**: Built-in schema validation for all entity operations
- **Complex actions**: Create complex entity actions with comprehensive specifications
- **Universal persistence**: Save/load to any supported format through xdata
- **Lazy initialization**: Optimize entity loading and access with lazy loading patterns
- **Async-first**: All entity operations are async for optimal performance
- **Core-Facade separation**: Core-lib (Rust future) handles entity operations, facade provides unified API
- **Multi-language facades**: Python/Rust/TS/Go facades wrapping shared Rust core

---

## Core-Facade Development Strategy

### Current Phase (0.x) - Python Implementation
- **Both core and facade in Python**: Full implementation in Python for rapid development
- **Unified codebase**: Core and facade can share implementation details
- **Rapid prototyping**: Fast iteration and testing of concepts
- **Python ecosystem**: Leverage existing Python libraries and tools
- **Single language**: Simplified development and maintenance

### Future Phase (3.x) - Rust Core + Multi-Language Facades
- **Rust core**: High-performance, memory-safe core implementation
- **Multi-language facades**: Python/Rust/TS/Go facades wrapping Rust core
- **Language-specific optimizations**: Each facade optimized for its target language
- **Shared performance**: All languages benefit from Rust's performance
- **Language ergonomics**: Each language gets idiomatic API design

### Development Guidelines
- **Design for separation**: Structure code to easily separate core from facade
- **Interface contracts**: Define clear interfaces between core and facade
- **Language-agnostic core**: Core should be implementable in any language
- **Facade flexibility**: Facades should be easily adaptable to different languages
- **Performance considerations**: Design with future Rust migration in mind

### Migration Strategy
- **Incremental migration**: Move core functionality to Rust gradually
- **Backward compatibility**: Maintain Python facades during transition
- **Testing strategy**: Comprehensive testing across all language combinations
- **Documentation updates**: Update all documentation for multi-language support
- **Community support**: Provide migration guides and examples

---

## Development Environment

### Multi-Language Support
- **Rust toolchains** - Support both GNU and MSVC Rust toolchains
- **Language compatibility** - Ensure Rust, Python, Go, and TypeScript all work flawlessly
- **Cross-platform** - Support multiple operating systems and architectures

### Development Tools
- **IDE integration** - Full IDE support with proper syntax highlighting and IntelliSense
- **Debugging** - Comprehensive debugging capabilities
- **Profiling** - Performance profiling and optimization tools

---

## Release & Publishing

### Publishing Strategy
- **GitHub Actions** - Use GitHub Actions for automated publishing
- **PyPI integration** - Store PyPI API tokens in GitHub secrets and use them automatically
- **Automated workflows** - Implement automated release and publishing workflows

### Quality Assurance
- **Pre-release testing** - Comprehensive testing before any release
- **Documentation updates** - Ensure all documentation is up-to-date
- **Version consistency** - Maintain version consistency across all components
- **Backward compatibility** - Consider backward compatibility implications

---

## Best Practices Summary

### Do's
? Think and design thoroughly before coding
? Use explicit imports throughout the project
? Include all formats and their dependencies
? Place documentation in docs/ folder
? Use pytest for all testing
? Keep versions exactly as specified
? Use libraries
? Implement comprehensive error handling
? Write clean, maintainable code
? Challenge ideas and assumptions
? **Fix root causes, not symptoms**
? **Follow 5 priorities when fixing errors**
? **Preserve all features when fixing bugs**

### Don'ts
? Never reinvent the wheel unnecessarily
? Don't use wildcard imports
? Don't change versions automatically
? Don't create backward compatibility aliases without confirmation
? Don't permanently delete files
? **Don't rig tests to pass - fix the root cause**
? **Don't use `pass` to silence errors**
? **Don't remove features to fix bugs**
? **Don't use workarounds instead of proper fixes**
? **Don't use `--disable-warnings` flag - it hides real problems!**
? **Don't use `--maxfail=10` - use `--maxfail=1` or `-x` instead**
? **Don't use `--tb=no` or `-q` to hide diagnostic output**
? **Don't add `filterwarnings = ignore` to pytest.ini**
? **Don't use `@pytest.mark.skip` or `@pytest.mark.xfail`**
? Don't create separate core.py files
? Don't put examples.py in src/ directory
? Don't mix Python versions
? **Don't skip root cause analysis - MANDATORY**
? **NEVER use "protocols.py" - always use "contracts.py" for interface files**

---

## Conclusion

These guidelines ensure consistent, high-quality development across all eXonware libraries and ALL work. They emphasize usability, maintainability, and quality while providing clear direction for both human developers and AI assistants.

### For Human Developers:
- **Reference this document** for all development decisions
- **Use as quality checklist** for reviewing AI-generated work
- **Maintain consistency** across all projects and libraries
- **Update guidelines** as new patterns and best practices emerge

### For AI Assistants:
- **Follow ALL guidelines** for any work - code, documentation, testing, project structure
- **Use this document as your quality standard** for all deliverables
- **Apply every rule and principle** consistently across all work
- **Ensure comprehensive compliance** with all sections of this document
- **⚠️ CRITICAL: When fixing errors, ALWAYS follow the Error Fixing Philosophy**
 - Never rig tests to pass
 - Never use `pass` to hide errors
 - Never remove features to eliminate bugs
 - Always fix root causes following the 5 priorities
 - Always preserve existing functionality
 - Always document WHY fixes were needed

**Remember**: The goal is to build enterprise-grade systems that are easy to use, maintain, and extend while maintaining the highest standards of security and performance. **When errors occur, fix them properly at the root cause - never use workarounds, never remove features, and never rig tests to pass.** This document serves as the comprehensive guide for achieving that goal.

---

*This document is living and should be updated as new patterns and best practices emerge. It serves as the definitive guide for all eXonware ecosystem development.*

