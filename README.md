# tauri2-noframe

基于 Tauri 2.0 + Vue 3 的无边框桌面应用模板。

## 特性

- 无边框窗口设计
- 自定义标题栏（支持拖拽、最小化、关闭）
- 启动时屏幕居中显示
- 禁止右键菜单和文本选中
- 禁止窗口最大化和调整大小
- 深色模式支持

## 技术栈

- **Tauri 2.0** - 跨平台桌面应用框架
- **Vue 3** - 前端框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具

## 开发

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建

```bash
npm run tauri build
```

## 项目结构

```
src/
├── App.vue              # 主应用组件
├── components/          # Vue 组件
│   └── TitleBar.vue     # 自定义标题栏组件
└── assets/              # 静态资源

src-tauri/
├── src/
│   ├── main.rs          # Rust 入口文件
│   └── lib.rs           # Tauri 命令定义
├── Cargo.toml           # Rust 依赖配置
└── tauri.conf.json      # Tauri 配置文件
```

## 配置说明

### 窗口配置

在 `src-tauri/tauri.conf.json` 中配置窗口属性：

```json
{
  "app": {
    "windows": [
      {
        "decorations": false, // 无边框
        "center": true, // 居中显示
        "resizable": false, // 禁止调整大小
        "maximizable": false // 禁止最大化
      }
    ]
  }
}
```

### 禁止右键和选中

在 `App.vue` 中：

- 使用 `@contextmenu.prevent` 禁止右键菜单
- 使用 `@selectstart.prevent` 禁止文本选中
- 使用 CSS `user-select: none` 禁止文本选中

## License

MIT
