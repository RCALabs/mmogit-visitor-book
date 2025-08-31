// Cloudflare Worker to proxy visitor book API requests
// Deploy this to visitor-book-api.mmogit.workers.dev

export default {
  async fetch(request, env, ctx) {
    // Handle preflight
    if (request.method === 'OPTIONS') {
      return new Response(null, {
        headers: {
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
          'Access-Control-Allow-Headers': 'Content-Type',
          'Access-Control-Max-Age': '86400',
        }
      });
    }

    try {
      // Proxy to our Hetzner server
      const originalUrl = new URL(request.url);
      const targetUrl = `http://api.mmogit.sh:3000${originalUrl.pathname}${originalUrl.search}`;
      
      // Note: Cloudflare Workers may block direct IP requests
      // Consider using a domain name for the backend
      const response = await fetch(targetUrl, {
        method: request.method,
        headers: {
          'Content-Type': request.headers.get('Content-Type') || 'application/json',
        },
        body: request.method !== 'GET' ? await request.text() : undefined,
      });

      // Add CORS headers
      const responseBody = await response.text();
      return new Response(responseBody, {
        status: response.status,
        statusText: response.statusText,
        headers: {
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
          'Access-Control-Allow-Headers': 'Content-Type',
          'Content-Type': response.headers.get('Content-Type') || 'application/json',
        }
      });
    } catch (error) {
      // If fetch fails (likely due to IP restriction), return error
      return new Response(JSON.stringify({
        error: 'Backend unavailable',
        message: 'Cloudflare Workers cannot reach IP addresses directly. Please use a domain name.',
        details: error.message
      }), {
        status: 503,
        headers: {
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
          'Access-Control-Allow-Headers': 'Content-Type',
          'Content-Type': 'application/json',
        }
      });
    }
  },
};