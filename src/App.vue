<script setup lang="ts">
import TitleBar from "./components/TitleBar.vue";
</script>

<template>
  <TitleBar />
  <main class="container" @contextmenu.prevent @selectstart.prevent></main>
</template>

<style scoped></style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;

  user-select: none;
  -webkit-user-select: none;
}

.container {
  margin: 0;
  padding-top: calc(32px + 10vh);
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>
