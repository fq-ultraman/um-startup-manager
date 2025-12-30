# UM 启动项管理器

[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-blue)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-2021-orange)](https://www.rust-lang.org/)

一个基于 Tauri 2.0 和 Vue 3 构建的现代化 Windows 启动项管理工具，专为无边框桌面应用设计。

## ✨ 特性

### 🎨 界面特性

- **无边框设计** - 现代化的无边框窗口体验
- **自定义标题栏** - 支持拖拽、最小化、关闭等操作
- **屏幕居中** - 启动时自动居中显示
- **深色模式** - 自动检测系统主题，完美适配
- **响应式设计** - 适配不同屏幕尺寸

### 🛡️ 安全特性

- **禁用右键菜单** - 防止意外操作
- **禁用文本选中** - 保持界面整洁
- **禁止窗口调整** - 固定窗口大小，防止误操作
- **禁止窗口最大化** - 确保应用始终保持设计尺寸

### ⚡ 性能特性

- **轻量级** - 基于 Tauri 的原生性能
- **快速启动** - 优化的启动流程
- **低内存占用** - 高效的资源管理

## 🛠️ 技术栈

| 技术           | 版本 | 用途               |
| -------------- | ---- | ------------------ |
| **Tauri**      | 2.0  | 跨平台桌面应用框架 |
| **Vue**        | 3.5  | 前端框架           |
| **TypeScript** | 5.6  | 类型安全           |
| **Rust**       | 2021 | 后端逻辑           |
| **Vite**       | 6.0  | 构建工具           |

## 🚀 快速开始

### 环境要求

- Node.js >= 18.0.0
- Rust >= 1.57.0
- Windows 10/11 (主要支持平台)

### 安装依赖

```bash
# 克隆项目
git clone <repository-url>
cd um-startup-manager

# 安装前端依赖
npm install

# 安装 Rust 依赖
cd src-tauri
cargo fetch
cd ..
```

### 开发模式

```bash
# 启动开发服务器
npm run tauri dev
```

### 构建生产版本

```bash
# 构建应用
npm run tauri build

# 构建后的文件位于
src-tauri/target/release/bundle/
```

## 📁 项目结构

```
um-startup-manager/
├── 📁 src/                          # Vue 前端源码
│   ├── 📁 components/               # Vue 组件
│   │   ├── 📄 TitleBar.vue         # 自定义标题栏
│   │   ├── 📄 StartupList.vue      # 启动项列表
│   │   └── 📄 StartupItem.vue      # 启动项项组件
│   ├── 📁 assets/                  # 静态资源
│   ├── 📄 App.vue                  # 主应用组件
│   └── 📄 main.ts                  # 应用入口
├── 📁 src-tauri/                    # Tauri 后端源码
│   ├── 📁 src/                     # Rust 源码
│   │   ├── 📄 main.rs              # Rust 入口文件
│   │   ├── 📄 lib.rs               # Tauri 命令定义
│   │   └── 📁 startup/             # 启动项管理模块
│   ├── 📄 Cargo.toml               # Rust 依赖配置
│   └── 📄 tauri.conf.json          # Tauri 配置文件
├── 📄 index.html                    # HTML 模板
├── 📄 package.json                  # NPM 依赖配置
├── 📄 vite.config.ts                # Vite 配置
└── 📄 README.md                     # 项目文档
```

## ⚙️ 配置说明

### 窗口配置

在 `src-tauri/tauri.conf.json` 中配置应用窗口属性：

```json
{
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "UM启动项管理",
        "width": 800,
        "height": 600,
        "decorations": false, // 无边框设计
        "center": true, // 屏幕居中
        "resizable": false, // 禁止调整大小
        "maximizable": false // 禁止最大化
      }
    ]
  }
}
```

### 安全配置

应用禁用以下功能以提升用户体验：

#### 禁用右键菜单

```vue
<template>
  <div @contextmenu.prevent>
    <!-- 内容 -->
  </div>
</template>
```

#### 禁用文本选中

```css
.app {
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}
```

### 主题配置

支持深色模式自动切换：

```typescript
// 自动检测系统主题
const isDarkMode = ref(
  window.matchMedia("(prefers-color-scheme: dark)").matches
);

// 监听主题变化
window
  .matchMedia("(prefers-color-scheme: dark)")
  .addEventListener("change", (e) => {
    isDarkMode.value = e.matches;
    document.documentElement.classList.toggle("dark", isDarkMode.value);
  });
```

## 🎯 核心功能

### 启动项管理

- 查看和管理系统启动项
- 启用/禁用启动项
- 删除不需要的启动项
- 添加新的启动项

### 设置管理

- 自动最小化设置
- 启动后最小化行为
- 进程名称映射

### 用户界面

- 直观的列表视图
- 搜索和过滤功能
- 批量操作支持

## 🔧 开发指南

### 添加新功能

1. **前端组件** (`src/components/`)

   - 创建新的 Vue 组件
   - 添加 TypeScript 类型定义
   - 实现响应式数据绑定

2. **后端命令** (`src-tauri/src/`)
   - 在 `lib.rs` 中定义新命令
   - 实现对应的 Rust 函数
   - 更新前端 API 调用

### 代码规范

- **Vue 组件**: 使用 Composition API
- **TypeScript**: 启用严格模式
- **Rust**: 遵循 Rust 最佳实践
- **CSS**: 使用 Tailwind CSS 样式

### 调试

```bash
# 前端调试
npm run dev

# 后端调试
npm run tauri dev

# 查看日志
# Windows: 查看应用事件日志
```

## 📦 构建和发布

### 构建配置

```bash
# 构建所有平台
npm run tauri build

# 构建特定平台
npm run tauri build --target x86_64-pc-windows-msvc
```

### 发布检查清单

- [ ] 更新版本号
- [ ] 测试所有功能
- [ ] 检查界面适配
- [ ] 验证安全配置
- [ ] 更新文档

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri Team](https://tauri.app/) - 优秀的桌面应用框架
- [Vue Team](https://vuejs.org/) - 优雅的前端框架
- [Rust Community](https://www.rust-lang.org/) - 系统级编程语言

## 📞 支持

如果您觉得这个项目有用，请给个 ⭐️ Star！

如有问题或建议，请提交 [Issue](../../issues)。

---

<div align="center">

**Made with ❤️ by UM Team**

[查看文档](https://github.com/your-username/um-startup-manager/wiki) • [报告问题](https://github.com/your-username/um-startup-manager/issues) • [功能请求](https://github.com/your-username/um-startup-manager/issues/new)

</div>
