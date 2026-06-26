// crm.js — AkurAI-CRM API client
// Native ESM, no dependencies

const API_BASE = '/api';

export class CRMClient {
  constructor(base = API_BASE) {
    this.base = base;
  }

  async request(method, path, body) {
    const opts = { method, headers: { 'Content-Type': 'application/json' } };
    if (body) opts.body = JSON.stringify(body);
    const res = await fetch(`${this.base}${path}`);
    if (!res.ok) {
      const err = await res.text();
      throw new Error(err || `HTTP ${res.status}`);
    }
    return res.json();
  }

  list(entity) { return this.request('GET', `/${entity}`); }
  get(entity, id) { return this.request('GET', `/${entity}/${id}`); }
  create(entity, data) { return this.request('POST', `/${entity}`, data); }
  update(entity, id, data) { return this.request('PUT', `/${entity}/${id}`, data); }
  delete(entity, id) { return this.request('DELETE', `/${entity}/${id}`); }

  search(query) { return this.request('GET', `/search?q=${encodeURIComponent(query)}`); }

  timeline(entityType, entityId) {
    return this.request('GET', `/timeline?entityType=${entityType}&entityId=${entityId}`);
  }

  meta() { return this.request('GET', '/_meta'); }
}

export const client = new CRMClient();
