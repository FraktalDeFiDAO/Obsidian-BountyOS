<script setup lang="ts">
const envVars = [
  { key: 'GITHUB_TOKEN', label: 'GitHub Token', description: 'GitHub API token for scanning issues' },
  { key: 'GITCOIN_API_KEY', label: 'Gitcoin API Key', description: 'Gitcoin API key' },
  { key: 'HACKERONE_API_KEY', label: 'HackerOne API Key', description: 'HackerOne API credentials' },
  { key: 'TELEGRAM_BOT_TOKEN', label: 'Telegram Bot Token', description: 'For notifications' },
  { key: 'DISCORD_WEBHOOK_URL', label: 'Discord Webhook', description: 'Discord webhook URL' },
]
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <h2 class="text-2xl font-bold mb-6">Settings</h2>

    <section class="mb-8">
      <h3 class="text-lg font-semibold mb-4 text-indigo-400">API Keys</h3>
      <div class="space-y-4">
        <div v-for="env in envVars" :key="env.key" class="card">
          <label class="block text-sm font-medium mb-1">{{ env.label }}</label>
          <input type="password" class="input" :placeholder="env.key" />
          <p class="text-xs text-slate-500 mt-1">{{ env.description }}</p>
        </div>
      </div>
    </section>

    <section class="mb-8">
      <h3 class="text-lg font-semibold mb-4 text-indigo-400">CLI Commands</h3>
      <div class="card font-mono text-sm">
        <p class="mb-2"># Scan all platforms</p>
        <p class="mb-2 text-indigo-400">obsidian-bounty-finder scan --all</p>
        <p class="mb-2"># Start API server</p>
        <p class="text-indigo-400">obsidian-bounty-finder serve</p>
      </div>
    </section>

    <button class="btn btn-primary">Save Settings</button>
  </div>
</template>
