<script setup>
import { ref, onMounted, onUnmounted } from "vue";

// 轮播状态管理
const currentSlide = ref(0);
const slides = ref([1, 2, 3]); // 3张幻灯片
let carouselInterval = null;

// 自动轮播间隔（毫秒）
const AUTO_PLAY_INTERVAL = 4000;

// 上一张幻灯片
const prevSlide = () => {
  currentSlide.value =
    (currentSlide.value - 1 + slides.value.length) % slides.value.length;
};

// 下一张幻灯片
const nextSlide = () => {
  currentSlide.value = (currentSlide.value + 1) % slides.value.length;
};

// 自动轮播
const startAutoPlay = () => {
  carouselInterval = setInterval(() => {
    nextSlide();
  }, AUTO_PLAY_INTERVAL);
};

// 停止自动轮播
const stopAutoPlay = () => {
  if (carouselInterval) {
    clearInterval(carouselInterval);
    carouselInterval = null;
  }
};

// 组件挂载时启动自动轮播
onMounted(() => {
  startAutoPlay();
});

// 组件卸载时停止自动轮播
onUnmounted(() => {
  stopAutoPlay();
});
</script>

<template>
  <div class="font-sans antialiased">
    <!-- Hero Section -->
    <section class="gradient-bg min-h-80 relative overflow-hidden">
      <!-- Navigation -->
      <nav class="absolute top-4 left-4 right-4 z-10">
        <div
          class="max-w-6xl mx-auto px-6 py-4 bg-white/5 backdrop-blur-md rounded-2xl border border-white/10"
        >
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <img
                src="/app-icon.png"
                alt="UM启动项管理"
                class="w-10 h-10 object-cover"
              />
              <span class="text-white font-semibold text-lg">UM启动项管理</span>
            </div>
            <div class="hidden md:flex items-center gap-8">
              <a
                href="#features"
                class="text-slate-300 hover:text-white transition-colors cursor-pointer"
                >功能</a
              >
              <a
                href="#preview"
                class="text-slate-300 hover:text-white transition-colors cursor-pointer"
                >预览</a
              >
              <a
                href="https://cnb.cool/ultraman-open/um-startup-manager/-/releases"
                target="_blank"
                class="bg-primary hover:bg-primary-dark text-white px-5 py-2 rounded-lg font-medium transition-colors cursor-pointer"
                >下载</a
              >
            </div>
          </div>
        </div>
      </nav>

      <!-- Hero Content -->
      <div class="max-w-6xl mx-auto px-6 pt-32 pb-20">
        <div class="text-center">
          <div
            class="inline-flex items-center gap-2 bg-primary/20 text-primary px-4 py-2 rounded-full text-sm font-medium mb-6"
          >
            <svg
              class="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            免费 · 轻量 · 开源
          </div>
          <h1
            class="text-4xl md:text-6xl font-bold text-white mb-6 leading-tight"
          >
            轻松管理<br class="md:hidden" /><span class="text-primary"
              >Windows启动项</span
            >
          </h1>
          <p class="text-slate-400 text-lg md:text-xl max-w-2xl mx-auto mb-10">
            支持任意软件启动后最小化，维持开机后桌面整洁。
          </p>
          <div class="flex flex-col sm:flex-row gap-4 justify-center">
            <a
              href="https://cnb.cool/ultraman-open/um-startup-manager/-/releases"
              target="_blank"
              class="inline-flex items-center justify-center gap-2 bg-primary hover:bg-primary-dark text-white px-8 py-4 rounded-xl font-semibold text-lg transition-colors cursor-pointer"
            >
              <svg
                class="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                />
              </svg>
              免费下载
            </a>
            <a
              href="https://github.com/fq-ultraman/um-startup-manager"
              target="_blank"
              class="inline-flex items-center justify-center gap-3 bg-white/10 hover:bg-white/20 text-white px-8 py-4 rounded-xl font-semibold text-lg transition-colors cursor-pointer border border-white/20"
            >
              <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                <path
                  d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                />
              </svg>
              GitHub 源码
            </a>
            <a
              href="https://cnb.cool/ultraman-open/um-startup-manager"
              target="_blank"
              class="inline-flex items-center justify-center gap-3 bg-white/10 hover:bg-white/20 text-white px-8 py-4 rounded-xl font-semibold text-lg transition-colors cursor-pointer border border-white/20"
            >
              <img
                src="https://docs.cnb.cool/images/logo/svg/Symbol-Color.svg"
                alt="CNB"
                class="w-6 h-6"
              />
              CNB 源码
            </a>
          </div>
        </div>

        <!-- App Preview Carousel -->
        <div id="preview" class="mt-16 relative">
          <div
            class="app-preview bg-dark-card rounded-2xl border border-white/10 overflow-hidden max-w-3xl mx-auto"
          >
            <!-- Carousel Container -->
            <div
              class="relative"
              style="aspect-ratio: 16/9.8; max-height: 700px"
            >
              <!-- Carousel Images -->
              <div
                class="carousel-track absolute inset-0 flex transition-transform duration-500 ease-in-out"
                :style="{ transform: `translateX(-${currentSlide * 100}%)` }"
              >
                <div class="carousel-item w-full flex-shrink-0 h-full">
                  <img
                    src="/Screenshot1.png"
                    alt="启动项管理界面"
                    class="w-full h-full object-contain"
                  />
                </div>
                <div class="carousel-item w-full flex-shrink-0 h-full">
                  <img
                    src="/Screenshot2.png"
                    alt="启动项详情"
                    class="w-full h-full object-contain"
                  />
                </div>
                <div class="carousel-item w-full flex-shrink-0 h-full">
                  <img
                    src="/Screenshot3.png"
                    alt="启动项设置"
                    class="w-full h-full object-contain"
                  />
                </div>
              </div>

              <!-- Previous Button -->
              <button
                @click="prevSlide"
                class="carousel-btn prev-btn absolute left-4 top-1/2 transform -translate-y-1/2 w-10 h-10 bg-white/10 backdrop-blur-sm rounded-full flex items-center justify-center text-white hover:bg-white/20 transition-colors"
              >
                <svg
                  class="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M15 19l-7-7 7-7"
                  />
                </svg>
              </button>

              <!-- Next Button -->
              <button
                @click="nextSlide"
                class="carousel-btn next-btn absolute right-4 top-1/2 transform -translate-y-1/2 w-10 h-10 bg-white/10 backdrop-blur-sm rounded-full flex items-center justify-center text-white hover:bg-white/20 transition-colors"
              >
                <svg
                  class="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 5l7 7-7 7"
                  />
                </svg>
              </button>

              <!-- Carousel Indicators -->
              <div
                class="carousel-indicators absolute bottom-4 left-1/2 transform -translate-x-1/2 flex gap-2"
              >
                <button
                  v-for="(slide, index) in slides"
                  :key="index"
                  @click="currentSlide = index"
                  class="indicator w-3 h-3 rounded-full transition-all duration-300"
                  :class="{
                    'bg-primary w-8': currentSlide === index,
                    'bg-white/30': currentSlide !== index,
                  }"
                ></button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features Section -->
    <section id="features" class="py-24 bg-slate-50">
      <div class="max-w-6xl mx-auto px-6">
        <div class="text-center mb-16">
          <h2 class="text-3xl md:text-4xl font-bold text-dark mb-4">
            核心功能
          </h2>
          <p class="text-secondary text-lg">简单易用，符合直觉</p>
        </div>
        <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          <!-- Feature 1 -->
          <div
            class="feature-card bg-white p-6 rounded-2xl border border-slate-200 transition-all duration-300 cursor-pointer hover:shadow-lg hover:border-primary/30"
          >
            <div
              class="w-14 h-14 bg-primary/10 rounded-xl flex items-center justify-center mb-5"
            >
              <svg
                class="w-7 h-7 text-primary"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"
                />
              </svg>
            </div>
            <h3 class="text-dark font-semibold text-lg mb-2">查看启动项</h3>
            <p class="text-secondary text-sm leading-relaxed">
              清晰展示所有系统启动项，包括程序名称、路径和状态
            </p>
          </div>
          <!-- Feature 2 -->
          <div
            class="feature-card bg-white p-6 rounded-2xl border border-slate-200 transition-all duration-300 cursor-pointer hover:shadow-lg hover:border-primary/30"
          >
            <div
              class="w-14 h-14 bg-accent/10 rounded-xl flex items-center justify-center mb-5"
            >
              <svg
                class="w-7 h-7 text-accent"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"
                />
              </svg>
            </div>
            <h3 class="text-dark font-semibold text-lg mb-2">启用/禁用</h3>
            <p class="text-secondary text-sm leading-relaxed">
              一键切换启动项状态，无需重启即可生效
            </p>
          </div>
          <!-- Feature 3 -->
          <div
            class="feature-card bg-white p-6 rounded-2xl border border-slate-200 transition-all duration-300 cursor-pointer hover:shadow-lg hover:border-primary/30"
          >
            <div
              class="w-14 h-14 bg-red-500/10 rounded-xl flex items-center justify-center mb-5"
            >
              <svg
                class="w-7 h-7 text-red-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                />
              </svg>
            </div>
            <h3 class="text-dark font-semibold text-lg mb-2">删除启动项</h3>
            <p class="text-secondary text-sm leading-relaxed">
              安全删除不需要的启动项，释放系统资源
            </p>
          </div>
          <!-- Feature 4 -->
          <div
            class="feature-card bg-white p-6 rounded-2xl border border-slate-200 transition-all duration-300 cursor-pointer hover:shadow-lg hover:border-primary/30"
          >
            <div
              class="w-14 h-14 bg-amber-500/10 rounded-xl flex items-center justify-center mb-5"
            >
              <svg
                class="w-7 h-7 text-amber-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M20 12H4m0 0l4-4m-4 4l4 4"
                />
              </svg>
            </div>
            <h3 class="text-dark font-semibold text-lg mb-2">自动最小化</h3>
            <p class="text-secondary text-sm leading-relaxed">
              启动时自动最小化指定程序，保持桌面整洁
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Download Section -->
    <section id="support" class="py-24 gradient-bg">
      <div class="max-w-4xl mx-auto px-6 text-center">
        <h2 class="text-3xl md:text-4xl font-bold text-white mb-4">支持开发</h2>
        <p class="text-slate-400 text-lg mb-10">
          如果您觉得这个项目有帮助，欢迎通过以下方式支持开发
        </p>
        <div
          class="flex flex-col md:flex-row gap-8 justify-center items-center"
        >
          <!-- 微信收款码 -->
          <div
            class="bg-white/10 backdrop-blur-md rounded-2xl p-6 border border-white/20"
          >
            <h3 class="text-white font-semibold text-xl mb-4">微信支付</h3>
            <div
              class="w-64 h-64 bg-white/5 rounded-xl flex items-center justify-center mb-4"
            >
              <!-- 微信收款码图片 -->
              <img
                src="/wechat.png"
                alt="微信收款码"
                class="w-full h-full object-contain"
              />
            </div>
            <p class="text-slate-400 text-sm">微信扫码支持</p>
          </div>

          <!-- 支付宝收款码 -->
          <div
            class="bg-white/10 backdrop-blur-md rounded-2xl p-6 border border-white/20"
          >
            <h3 class="text-white font-semibold text-xl mb-4">支付宝</h3>
            <div
              class="w-64 h-64 bg-white/5 rounded-xl flex items-center justify-center mb-4"
            >
              <!-- 支付宝收款码图片 -->
              <img
                src="/alipay.png"
                alt="支付宝收款码"
                class="w-full h-full object-contain"
              />
            </div>
            <p class="text-slate-400 text-sm">支付宝扫码支持</p>
          </div>
        </div>
        <p class="text-slate-500 text-sm mt-12">感谢您的支持与理解！</p>
      </div>
    </section>

    <!-- Footer -->
    <footer class="bg-dark py-8 border-t border-white/10">
      <div class="max-w-6xl mx-auto px-6">
        <div
          class="flex flex-col md:flex-row items-center justify-between gap-4"
        >
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 flex items-center justify-center">
              <img
                src="/app-icon.png"
                alt="UMStartupManager"
                class="w-8 h-8 object-contain"
              />
            </div>
            <span class="text-slate-400 text-sm">UMStartupManager</span>
          </div>
          <p class="text-slate-500 text-sm">
            © 2025 UM启动项管理. All rights reserved.
          </p>
        </div>
      </div>
    </footer>
  </div>
</template>

<style>
.gradient-bg {
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
}
.feature-card:hover {
  transform: translateY(-4px);
}
.app-preview {
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
}
</style>
