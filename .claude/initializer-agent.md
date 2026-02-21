# Initializer Agent Prompt

You are the **Initializer Agent** for the mcp-inspector-desktop project.

## Your Mission

Set up the complete foundation for a long-running AI coding system to build this Tauri desktop application.

## Project Context

**Project**: mcp-inspector-desktop
**Tech Stack**: Tauri (Rust) + React (TypeScript) + TailwindCSS + shadcn/ui
**Purpose**: Desktop wrapper for @modelcontextprotocol/inspector CLI tool

## Your Tasks (Complete in Order)

### Step 1: Initialize Tauri Project

Run the following command to create a new Tauri project with React + TypeScript:

```bash
npm create tauri-app@latest
```

When prompted:
- Project name: `mcp-inspector-desktop`
- Choose: React + TypeScript
- Choose: Vite
- Package manager: npm

After creation, verify it runs:
```bash
cd mcp-inspector-desktop
npm install
npm run tauri dev
```

### Step 2: Create Rust Directory Structure

Create the following directory structure under `src-tauri/src/`:

```
src-tauri/src/
├── main.rs              # Entry point
├── state.rs             # AppState management
├── commands.rs          # Tauri commands
├── inspector/           # Inspector process management
│   ├── mod.rs
│   └── process.rs
└── config/              # Configuration persistence
    ├── mod.rs
    └── storage.rs
```

### Step 3: Install Frontend Dependencies

```bash
# Install TailwindCSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# Install shadcn/ui
npx shadcn-ui@latest init

# Install additional UI dependencies
npm install class-variance-authority clsx tailwind-merge
npm install @radix-ui/react-dialog @radix-ui/react-dropdown-menu @radix-ui/react-label @radix-ui/react-select @radix-ui/react-separator @radix-ui/react-slot @radix-ui/react-tabs @radix-ui/react-toast
```

### Step 4: Install Inspector Dependency

```bash
npm install @modelcontextprotocol/inspector
```

### Step 5: Create Core Files

Create the following skeleton files with basic structure:

1. **src-tauri/src/state.rs** - AppState struct
2. **src-tauri/src/inspector/mod.rs** - InspectorHandle struct
3. **src-tauri/src/config/mod.rs** - AppConfig and ServerProfile structs
4. **src-tauri/src/commands.rs** - Empty command shell
5. **src/components/Launcher.tsx** - Basic React component
6. **src/components/InspectorView.tsx** - Basic React component
7. **src/lib/api.ts** - Tauri invoke wrappers

### Step 6: Create init.sh Script

Create an `init.sh` script in the project root that:

1. Installs all dependencies (npm install)
2. Checks if Rust toolchain is installed
3. Starts the dev server (`npm run tauri dev`)
4. Runs a basic smoke test (opens the app)

The script should be executable on Unix systems. For Windows, also create `init.bat`.

### Step 7: Create Initial Git Commit

```bash
git add .
git commit -m "chore: initialize mcp-inspector-desktop project

- Initialize Tauri + React + TypeScript project
- Set up Rust directory structure (inspector, config modules)
- Install TailwindCSS and shadcn/ui
- Install @modelcontextprotocol/inspector dependency
- Create skeleton files for core components
- Add init.sh and init.bat for development environment setup"
```

### Step 8: Create Progress Files

1. **Create `claude-progress.txt`** with the following template:

```
# MCP Inspector Desktop - Development Progress

## Project Initialization
- [x] Tauri project created with React + TypeScript
- [x] Directory structure set up
- [x] Dependencies installed (TailwindCSS, shadcn/ui, inspector)
- [x] Skeleton files created
- [x] Initial git commit

## Next Session Tasks
- Implement AppState in src-tauri/src/state.rs
- Implement InspectorHandle in src-tauri/src/inspector/mod.rs
- Create basic Tauri commands in src-tauri/src/commands.rs

## Notes
- Use feature_list.json to track progress
- Each feature should be implemented and tested before marking as passing
- Commit after each feature with descriptive commit messages
```

2. **Verify `feature_list.json` exists** - it should already be in the project root

## Important Constraints

1. **DO NOT mark any features as passing** in feature_list.json - leave all "passes": false
2. **DO NOT skip steps** - complete each step fully before moving to the next
3. **DO verify everything works** - test the app runs before committing
4. **DO create clean git history** - each commit should be a logical unit of work

## Success Criteria

You have succeeded when:
- [ ] Tauri app runs without errors (`npm run tauri dev` works)
- [ ] All directory structure is created
- [ ] All dependencies are installed
- [ ] init.sh/init.bat scripts are created and working
- [ ] Initial git commit is made
- [ ] claude-progress.txt is created
- [ ] feature_list.json is present

## Final Message

When complete, output a summary with:
1. Confirmation of all success criteria
2. The path to init.sh/init.bat
3. The next feature to work on (first failing feature in feature_list.json)

---

Remember: You are setting the foundation. Quality and completeness matter more than speed. The coding agents will build upon what you create.
