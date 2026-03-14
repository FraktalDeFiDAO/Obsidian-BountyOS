<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <div class="min-h-screen bg-slate-900 text-white">
    <header class="bg-slate-800 border-b border-slate-700">
      <div class="container mx-auto px-4 py-4 flex items-center justify-between">
        <h1 class="text-xl font-bold text-indigo-400">ObsidianBountyFinder</h1>
        <nav class="flex gap-4">
          <RouterLink to="/" class="hover:text-indigo-400">Home</RouterLink>
          <RouterLink to="/bounties" class="hover:text-indigo-400">Bounties</RouterLink>
          <RouterLink to="/wallet" class="hover:text-indigo-400">Wallet</RouterLink>
          <RouterLink to="/settings" class="hover:text-indigo-400">Settings</RouterLink>
        </nav>
      </div>
    </header>
    <main class="flex-1">
      <RouterView />
    </main>
    <footer class="bg-slate-800 border-t border-slate-700 py-4">
      <div class="container mx-auto px-4 text-center text-slate-400">
        <p>ObsidianBountyFinder - Unified bounty tracker</p>
      </div>
    </footer>
  </div>
</template>
