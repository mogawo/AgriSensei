const username = document.getElementById('usernameInput');
const password = document.getElementById('passwordInput');
const login = document.getElementById('login');
const signup = document.getElementById('signup');
const body = document.querySelector('.body');

var users =
{
    "Yareli": '',
    "Sophie": '',
    "Winston": '',
    "Norman": '',
    "Kimberly": '',
    "Kara": '',
    "Juan": '',
    "Billy": '',
    "Braulio": '',
    "Damien": '',
    "Ezra": '',
    "Margarita": '',
    "Gisselle": '',
    "Leeann": '',
    "Davis": '',
    "Alex": '',
    "Justin": '',
    "Kenna": '',
    "Jorden": '',
    "Remy": '',
};

console.log(login);



function handleSignUp()
{
    const existingResponse = document.querySelector('.response');
    if (existingResponse != null)
    {
        existingResponse.remove();    
    }
    
    console.log("Pressed Sign Up");
    const response = document.createElement('div'); 
    response.classList.add('response');
    for (var i = 0; i < username.value.length; i++)
    {       
        if (username.value.charCodeAt(i) > 127 || username.value.charCodeAt(i) < 0)
        {
            response.innerHTML = `
                <p>Invalid username<p>
            `;
            body.appendChild(response);   
            username.value = '';
            password.value = '';
            return;
        }
    }
    if (username.value in users)
    {
        response.innerHTML = `
            <p>Username is already in use<p>
        `;
        body.appendChild(response);
        username.value = '';
        password.value = '';
        return;
    }
    else if (username.value == '')
    {
        response.innerHTML = `
            <p>Please input a username<p>
        `;
        body.appendChild(response);
        username.value = '';
        password.value = '';
        return;
    }
    response.innerHTML = `
        <p>Account has been successfully registered<p>
    `;  
    body.appendChild(response);
    console.log("Is valid user");
    users[username.value] = password.value;
    username.value = '';
    password.value = '';
}

function handleLogIn() {
    if (username.value in users)
    {
        window.location = "../main_page/index.html"; 
        if (password.value == users[username.value] || password.value == '')
        {
            setUser(username.value);
            window.location = "../main_page/index.html"; 
        }
    }
    console.log("Invalid Username or Password");
    username.value = '';
    password.value = '';
}