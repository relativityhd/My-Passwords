import { defineConfig } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite'
import { internalIpV4 } from 'internal-ip'

// https://vitejs.dev/config/
export default defineConfig(async () => {
  const host = await internalIpV4()
  console.log(`Vite dev server running at http://${host}:5173`)

  /** @type {import('vite').UserConfig} */
  const config = {
    plugins: [sveltekit()],
    test: {
      include: ['src/**/*.{test,spec}.{js,ts}'],
    },
    server: {
      host: '0.0.0.0', // listen on all addresses
      port: 5173,
      strictPort: true,
      hmr: {
        protocol: 'ws',
        host: '0.0.0.0', // listen on all addresses (was host from internalIpV4() above, but that didn't work for me in WSL2)
        port: 5183,
      },
    },
  }

  return config
})