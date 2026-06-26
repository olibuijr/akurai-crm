// databinder.js — Client-side data binding for CRM pages
// Fetches from API and renders into DOM

import { client } from './crm.js';

export async function loadTable(entity, tableBodyId) {
  const data = await client.list(entity);
  const tbody = document.getElementById(tableBodyId);
  if (!tbody || !data || !data.length) {
    if (tbody) tbody.innerHTML = '<tr><td colspan="10" class="empty-state">No records found</td></tr>';
    return data || [];
  }
  return data;
}

export async function loadStats() {
  const [people, companies, opportunities, tasks] = await Promise.all([
    client.list('people').catch(() => []),
    client.list('companies').catch(() => []),
    client.list('opportunities').catch(() => []),
    client.list('tasks').catch(() => []),
  ]);
  setText('stat-contacts', (people.length + companies.length).toString());
  setText('stat-deals', opportunities.filter(o => o.stage !== 'won' && o.stage !== 'lost').length.toString());
  setText('stat-revenue', '$' + opportunities.filter(o => o.stage !== 'lost').reduce((s, o) => s + (o.amount || 0), 0).toLocaleString());
  setText('stat-tasks', tasks.filter(t => t.status !== 'done').length.toString());
}

export function renderPeopleTable(data, tbodyId) {
  const tbody = document.getElementById(tbodyId);
  if (!tbody) return;
  tbody.innerHTML = data.map(p => `
    <tr>
      <td><a href="/person-detail.html?id=${p.id}" class="person-cell">
        <div class="avatar avatar-sm">${(p.firstName?.[0]||'')}${(p.lastName?.[0]||'')}</div>
        ${p.firstName || ''} ${p.lastName || ''}
      </a></td>
      <td class="cell-muted">${p.email || '—'}</td>
      <td class="cell-muted">${p.phone || '—'}</td>
      <td class="cell-muted">${p.jobTitle || '—'}</td>
      <td class="cell-muted">${p.companyId ? `<a href="/company-detail.html?id=${p.companyId}">Company #${p.companyId}</a>` : '—'}</td>
    </tr>
  `).join('');
  updatePagination(tbodyId, data.length);
}

export function renderCompaniesTable(data, tbodyId) {
  const tbody = document.getElementById(tbodyId);
  if (!tbody) return;
  tbody.innerHTML = data.map(c => `
    <tr>
      <td><a href="/company-detail.html?id=${c.id}" class="person-cell">${c.name || 'Unnamed'}</a></td>
      <td class="cell-muted">${c.domainName || '—'}</td>
      <td class="cell-muted">${c.annualRevenue ? '$' + (c.annualRevenue / 100).toLocaleString() : '—'}</td>
      <td class="cell-muted">${c.employeeCount || '—'}</td>
      <td class="cell-muted">${c.websiteUrl ? `<a href="${c.websiteUrl}" target="_blank">${c.websiteUrl}</a>` : '—'}</td>
    </tr>
  `).join('');
  updatePagination(tbodyId, data.length);
}

export function renderOpportunitiesTable(data, tbodyId) {
  const tbody = document.getElementById(tbodyId);
  if (!tbody) return;
  tbody.innerHTML = data.map(o => {
    const stageClass = `badge badge-${o.stage || 'new'}`;
    return `<tr>
      <td><a href="/opportunity-detail.html?id=${o.id}">${o.name || 'Unnamed'}</a></td>
      <td class="cell-muted">${o.amount ? '$' + (o.amount / 100).toLocaleString() : '—'}</td>
      <td><span class="${stageClass}">${o.stage || 'new'}</span></td>
      <td class="cell-muted">${o.probability != null ? o.probability + '%' : '—'}</td>
      <td class="cell-muted">${o.personId ? `Person #${o.personId}` : '—'}</td>
    </tr>`;
  }).join('');
  updatePagination(tbodyId, data.length);
}

export function renderTasksList(data, listId) {
  const list = document.getElementById(listId);
  if (!list) return;
  list.innerHTML = data.map(t => {
    const isDone = t.status === 'done';
    return `<div class="task-item${isDone ? ' done' : ''}" data-status="${t.status || 'todo'}">
      <input type="checkbox" class="task-checkbox"${isDone ? ' checked' : ''}>
      <span class="task-title">${t.title || 'Untitled'}</span>
      <div class="task-meta">
        <span class="task-due">${t.dueDate || '—'}</span>
        <span class="badge badge-${isDone ? 'green' : 'blue'}">${t.relatedTo || t.status || 'todo'}</span>
      </div>
    </div>`;
  }).join('');
  updatePagination(listId, data.length);
}

function setText(id, text) {
  const el = document.getElementById(id);
  if (el) el.textContent = text;
}

function updatePagination(tbodyId, total) {
  const parent = document.getElementById(tbodyId)?.closest('.card') || document;
  const pagination = parent.querySelector('.pagination');
  if (pagination) {
    const info = pagination.querySelector('.pagination-info');
    if (info) info.textContent = `Showing all ${total} records`;
  }
}

// Render person detail page
export async function loadPersonDetail(personId) {
  if (!personId) return;
  try {
    const p = await client.get('people', personId);
    setText('detail-name', `${p.firstName || ''} ${p.lastName || ''}`);
    setText('detail-jobtitle', p.jobTitle || '—');
    setText('detail-email', p.email || '—');
    setText('detail-phone', p.phone || '—');
    setText('detail-company', p.companyId ? `Company #${p.companyId}` : '—');
    if (p.firstName) {
      const avatar = document.querySelector('#detail-avatar');
      if (avatar) avatar.textContent = (p.firstName[0] || '') + (p.lastName?.[0] || '');
    }
  } catch(e) {
    console.warn('Failed to load person detail:', e);
  }
}

// Render company detail page
export async function loadCompanyDetail(companyId) {
  if (!companyId) return;
  try {
    const c = await client.get('companies', companyId);
    setText('detail-company-name', c.name || 'Unnamed');
    setText('detail-domain', c.domainName || '—');
    setText('detail-revenue', c.annualRevenue ? '$' + (c.annualRevenue / 100).toLocaleString() : '—');
    setText('detail-employees', c.employeeCount?.toString() || '—');
    setText('detail-website', c.websiteUrl || '—');

    // Load people at this company
    const people = await client.list('people');
    const employees = people.filter(p => p.companyId == companyId);
    const tbody = document.getElementById('company-people-tbody');
    if (tbody) {
      tbody.innerHTML = employees.map(p => `
        <tr>
          <td><a href="/person-detail.html?id=${p.id}">${p.firstName || ''} ${p.lastName || ''}</a></td>
          <td class="cell-muted">${p.email || '—'}</td>
          <td class="cell-muted">${p.jobTitle || '—'}</td>
        </tr>
      `).join('') || '<tr><td colspan="3" class="empty-state">No contacts at this company</td></tr>';
    }
  } catch(e) {
    console.warn('Failed to load company detail:', e);
  }
}

// Render opportunity detail page
export async function loadOpportunityDetail(oppId) {
  if (!oppId) return;
  try {
    const o = await client.get('opportunities', oppId);
    setText('detail-deal-name', o.name || 'Unnamed');
    setText('detail-deal-amount', o.amount ? '$' + (o.amount / 100).toLocaleString() : '—');
    setText('detail-deal-stage', o.stage || '—');
    setText('detail-deal-probability', o.probability != null ? o.probability + '%' : '—');
    setText('detail-deal-person', o.personId ? `Person #${o.personId}` : '—');
    setText('detail-deal-company', o.companyId ? `Company #${o.companyId}` : '—');
  } catch(e) {
    console.warn('Failed to load opportunity detail:', e);
  }
}

// Render notes grid
export async function loadNotes() {
  try {
    const notes = await client.list('notes');
    const grid = document.getElementById('notes-grid');
    if (!grid) return;
    grid.innerHTML = notes.map(n => `
      <div class="card note-card">
        <div class="note-card-header">
          <h3 class="note-card-title">${n.title || 'Untitled Note'}</h3>
          <span class="text-xs text-muted">${new Date((n.createdAt || 0) * 1000).toLocaleDateString()}</span>
        </div>
        <div class="note-card-preview">${(n.body || '').substring(0, 120)}${(n.body || '').length > 120 ? '...' : ''}</div>
      </div>
    `).join('');
  } catch(e) {
    console.warn('Failed to load notes:', e);
  }
}

// Render kanban board from API data
export async function loadKanban() {
  try {
    const opps = await client.list('opportunities');
    const stages = ['new', 'screening', 'meeting', 'proposal', 'negotiation', 'won', 'lost'];
    const stageLabels = { new: 'New', screening: 'Screening', meeting: 'Meeting', proposal: 'Proposal', negotiation: 'Negotiation', won: 'Won', lost: 'Lost' };

    stages.forEach(stage => {
      const col = document.getElementById(`kanban-${stage}`);
      if (!col) return;
      const cards = opps.filter(o => (o.stage || 'new') === stage);
      col.innerHTML = cards.map(o => `
        <a href="/opportunity-detail.html?id=${o.id}" class="kanban-card" style="text-decoration:none;color:inherit;border-left-color: ${getStageColor(stage)};">
          <div class="card-title">${o.name || 'Unnamed'}</div>
          <div class="card-subtitle">${o.amount ? '$' + (o.amount / 100).toLocaleString() : '—'}</div>
          <div class="card-footer">
            <div class="card-amount">${o.amount ? '$' + (o.amount / 100).toLocaleString() : '—'}</div>
            <span class="badge badge-${stage}">${stageLabels[stage] || stage}</span>
          </div>
        </a>
      `).join('');

      // Update column header count
      const header = col.closest('.kanban-column')?.querySelector('.kanban-count');
      if (header) header.textContent = cards.length.toString();
    });
  } catch(e) {
    console.warn('Failed to load kanban:', e);
  }
}

function getStageColor(stage) {
  const colors = { new: '#94a3b8', screening: '#60a5fa', meeting: '#818cf8', proposal: '#a78bfa', negotiation: '#f59e0b', won: '#22c55e', lost: '#ef4444' };
  return colors[stage] || '#94a3b8';
}

// Auto-init: run stat loading on dashboard
if (window.location.pathname.includes('dashboard')) {
  loadStats();
}
