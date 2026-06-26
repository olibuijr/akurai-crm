// app.js — AkurAI-CRM Application Shell
import { client } from './crm.js';

document.addEventListener('DOMContentLoaded', () => {
  const path = window.location.pathname;
  document.querySelectorAll('.nav-link').forEach(link => {
    const href = link.getAttribute('href');
    if (path === href || (href !== '/' && path.startsWith(href))) {
      link.classList.add('active');
    } else {
      link.classList.remove('active');
    }
  });

  // Load meta info
  if (document.getElementById('meta-info')) {
    client.meta().then(data => {
      const el = document.getElementById('meta-info');
      if (data && data.entities) {
        el.textContent = `Connected — ${data.entities.length} entity types available`;
      }
    }).catch(() => {});
  }

  // Global notification toggle
  const notifBtn = document.getElementById('notif-btn');
  if (notifBtn) {
    notifBtn.addEventListener('click', () => {
      showToast('No new notifications', 'info');
    });
  }

  // Avatar dropdown
  const avatarBtn = document.getElementById('avatar-btn');
  if (avatarBtn) {
    avatarBtn.addEventListener('click', () => {
      window.location.href = '/settings.html';
    });
  }
});

export async function loadDashboardStats() {
  try {
    const [people, companies, opportunities, tasks] = await Promise.all([
      client.list('people').catch(() => []),
      client.list('companies').catch(() => []),
      client.list('opportunities').catch(() => []),
      client.list('tasks').catch(() => []),
    ]);
    const totalContacts = (people.length || 0) + (companies.length || 0);
    const activeDeals = (opportunities || []).filter(o => o.stage !== 'won' && o.stage !== 'lost').length;
    const pipelineRevenue = (opportunities || [])
      .filter(o => o.stage !== 'lost')
      .reduce((sum, o) => sum + (o.amount || 0), 0);
    const dueToday = (tasks || []).filter(t => t.status !== 'done').length;

    const statContacts = document.getElementById('stat-contacts');
    const statDeals = document.getElementById('stat-deals');
    const statRevenue = document.getElementById('stat-revenue');
    const statTasks = document.getElementById('stat-tasks');

    if (statContacts) statContacts.textContent = totalContacts;
    if (statDeals) statDeals.textContent = activeDeals;
    if (statRevenue) statRevenue.textContent = `$${(pipelineRevenue / 100).toLocaleString()}`;
    if (statTasks) statTasks.textContent = dueToday;
  } catch (e) {
    showToast('Could not load dashboard stats', 'error');
  }
}

export function showToast(message, type = 'info') {
  const container = document.getElementById('toast-container');
  if (!container) return;

  const icons = {
    success: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>',
    error: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>',
    warning: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>',
    info: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>',
  };

  const toast = document.createElement('div');
  toast.className = `toast toast-${type}`;
  toast.innerHTML = `
    <span class="toast-icon">${icons[type] || icons.info}</span>
    <span class="toast-message">${message}</span>
    <button class="toast-close" onclick="this.parentElement.remove()">&times;</button>
  `;
  container.appendChild(toast);

  setTimeout(() => {
    toast.style.animation = 'toastOut 0.2s ease forwards';
    setTimeout(() => toast.remove(), 200);
  }, 4000);
}

export function formatDate(dateStr) {
  if (!dateStr) return '—';
  const d = new Date(dateStr);
  return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
}

export function formatCurrency(amount) {
  if (amount == null) return '—';
  return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', minimumFractionDigits: 0, maximumFractionDigits: 0 }).format(amount);
}

export function getInitials(name) {
  if (!name) return '?';
  return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2);
}

export function stageBadgeClass(stage) {
  const map = {
    new: 'badge-slate',
    screening: 'badge-blue',
    meeting: 'badge-indigo',
    proposal: 'badge-violet',
    negotiation: 'badge-amber',
    won: 'badge-green',
    lost: 'badge-red',
  };
  return map[stage] || 'badge-gray';
}

export function stageLabel(stage) {
  if (!stage) return 'New';
  return stage.charAt(0).toUpperCase() + stage.slice(1);
}
