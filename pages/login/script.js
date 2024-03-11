const username = document.getElementById('usernameInput');
const password = document.getElementById('passwordInput');
const login = document.getElementById('login');

var users =
{
    'username': 'password',
    'otheruser': 'otherpass',
    'validuser': 'validpassword',
};

console.log(login);

login.addEventListener('click', () => {
    if (username.value in users)
    {
        if (password.value == users[username.value])
        {
            window.location = "../main_page/index.html"; 
        }
    }
    console.log("Invalid Username or Password");
    username.value = '';
    password.value = '';
});
