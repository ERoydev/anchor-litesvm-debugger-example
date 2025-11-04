

# Setup to run TS litesvm tests
You need to create a symbolic link of the node-litesmv which allows you to write litesvm in TS

### 1. Build an enhanced version of LiteSVM 0.8.1 that supports Gimlet:

Please clone this.

```bash
git clone https://github.com/procdump/litesvm
cd litesvm/crates/node-litesvm && yarn && yarn build -- --features sbpf-debugger
```

Finally in order for the Typescript tests to use the enhanced version of LiteSVM we've just built,
create a symbolic link to it at the root directory of the anchor workspace. Be sure to use full path:

```bash
ln -s /path/to/enhanced/litesvm/crates/node-litesvm/litesvm local-litesvm
```

### 2. Start Debugging

Go into your test cases — either `tests/anchor_cpi.ts` or `programs/program_a/tests/test_cpi.rs`.

Above the test cases, there should be a **`Sbpf Debug`** or **`Sbpf Debug All`** CodeLens button.

1. After clicking the button, it will start compiling your program using `anchor build`, and then launch the **LLDB debugger**, where you can begin debugging.  
2. It is recommended to use only the **`continue`** command to skip stepping into built-in code and focus on debugging your own program.

### 3. Test with Native Apps

In the workspace, you can find a **`native_apps`** directory where you can also test the debugger.

Make sure to open the specific project folder (e.g., one inside native_apps) as your workspace root in VS Code — Gimlet requires the project to be opened at the root level to detect and run correctly.