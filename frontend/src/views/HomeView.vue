<script setup lang="ts">
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <section class="mb-12">
      <h2 class="text-3xl font-bold mb-4">Welcome to ObsidianBountyFinder</h2>
      <p class="text-slate-400 mb-6">
        Track all your bounties, freelance opportunities, and bug bounty programs in one place.
      </p>
      <div class="flex gap-4">
        <RouterLink to="/bounties" class="btn btn-primary">
          View Bounties
        </RouterLink>
        <RouterLink to="/settings" class="btn bg-slate-700 hover:bg-slate-600">
          Configure
        </RouterLink>
      </div>
    </section>

    <section class="grid md:grid-cols-3 gap-6">
      <div class="card">
        <h3 class="text-lg font-semibold mb-2 text-indigo-400">Multi-Platform</h3>
        <p class="text-slate-400">GitHub, Gitcoin, HackerOne, Bugcrowd, and more</p>
      </div>
      <div class="card">
        <h3 class="text-lg font-semibold mb-2 text-indigo-400">Real-Time Updates</h3>
        <p class="text-slate-400">Webhooks, notifications, and live data</p>
      </div>
      <div class="card">
        <h3 class="text-lg font-semibold mb-2 text-indigo-400">CLI First</h3>
        <p class="text-slate-400">Powerful terminal interface for developers</p>
      </div>
    </section>
  </div>
</template>
