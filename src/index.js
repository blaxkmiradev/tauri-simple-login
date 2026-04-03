import { invoke } from '@tauri-apps/api/tauri';

const form = document.getElementById('loginForm');
const message = document.getElementById('message');

form.addEventListener('submit', async (e) => {
  e.preventDefault();
  const username = document.getElementById('username').value;
  const password = document.getElementById('password').value;

  try {
    const success = await invoke('login', { username, password });
    message.textContent = success ? 'Login successful!' : 'Invalid credentials.';
  } catch (err) {
    message.textContent = 'Error: ' + err;
  }
});
