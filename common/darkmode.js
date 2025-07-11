const toggle = document.getElementById('darkModeToggle');
toggle.addEventListener('click', () => {
    document.documentElement.classList.toggle('dark');
    const mode = document.documentElement.classList.contains('dark') ? 'dark' : 'light';
    localStorage.setItem('theme', mode);
});

if (localStorage.getItem('theme') === 'dark') {
    document.documentElement.classList.add('dark');
}