import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface WalletState {
  address: string
  chain: string
  connected: boolean
  balance: string
}

export interface TokenBalance {
  symbol: string
  balance: string
  value: number
}

export const useWalletStore = defineStore('wallet', () => {
  const connected = ref(false)
  const address = ref('')
  const chain = ref('')
  const balances = ref<TokenBalance[]>([])
  const loading = ref(false)

  const shortAddress = computed(() => {
    if (!address.value) return ''
    return `${address.value.slice(0, 6)}...${address.value.slice(-4)}`
  })

  async function connect() {
    loading.value = true
    try {
      // Demo wallet connection
      address.value = '0x742d35Cc6634C0532925a3b844Bc9e7595f8E3b1'
      chain.value = 'Ethereum'
      connected.value = true
      balances.value = [
        { symbol: 'ETH', balance: '2.5', value: 5000 },
        { symbol: 'USDC', balance: '1000', value: 1000 },
        { symbol: 'SOL', balance: '50', value: 2500 },
      ]
    } finally {
      loading.value = false
    }
  }

  function disconnect() {
    connected.value = false
    address.value = ''
    chain.value = ''
    balances.value = []
  }

  const totalValue = computed(() => {
    return balances.value.reduce((sum, t) => sum + t.value, 0)
  })

  return { connected, address, chain, balances, loading, shortAddress, totalValue, connect, disconnect }
})
