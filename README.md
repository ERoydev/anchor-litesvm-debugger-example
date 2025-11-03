

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
