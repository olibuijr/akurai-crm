// crm.js — AkurAI-CRM API client
// Native ESM, no dependencies

const API_BASE = '/api';

export class CRMClient {
  constructor(base = API_BASE) {
    this.base = base;
    this._token = null;
    this._restoreToken();
  }

  _restoreToken() {
    // 1. Check localStorage
    let token = localStorage.getItem('crm_session');
    if (token) { this._token = token; return; }
    // 2. Check cookie (fresh from OIDC callback redirect)
    token = this._cookieValue('crm_session');
    if (token) {
      this._token = token;
      localStorage.setItem('crm_session', token);
      // Remove the cookie so it's not sent on every request
      document.cookie = 'crm_session=; Path=/; Max-Age=0';
    }
  }

  _cookieValue(name) {
    const match = document.cookie.match(new RegExp('(?:^|;\\s*)' + name + '=([^;]*)'));
    return match ? decodeURIComponent(match[1]) : null;
  }

  get token() { return this._token; }

  clearToken() {
    this._token = null;
    localStorage.removeItem('crm_session');
    document.cookie = 'crm_session=; Path=/; Max-Age=0';
  }

  async request(method, path, body) {
    const opts = { method, headers: { 'Content-Type': 'application/json' } };
    if (this._token) opts.headers['Authorization'] = `Bearer ${this._token}`;
    if (body) opts.body = JSON.stringify(body);
    const res = await fetch(`${this.base}${path}`, opts);
    if (res.status === 401) {
      this.clearToken();
      window.location.href = '/?session_expired=1';
      throw new Error('Session expired');
    }
    if (!res.ok) {
      const err = await res.text().catch(() => '');
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

  me() { return this.request('GET', '/me'); }
}

export const client = new CRMClient();
