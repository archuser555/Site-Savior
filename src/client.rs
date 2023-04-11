// My poor mind can't think of another better method to do html & rust 
pub fn main_page() -> String {
    "<!DOCTYPE html>
        <html>
        <head>
            <meta charset='UTF-8'>
            <title>Site Savior - Your Swiss Army Knife for Managing Your Servers</title>
            <link href='https://fonts.googleapis.com/css?family=Roboto:400,700&display=swap' rel='stylesheet'>
            <style>
                body {
                    font-family: 'Roboto', sans-serif;
                    background-color: #1a1a1a;
                    color: #f5f5f5;
                    text-align: center;
                    margin: 0;
                    padding: 0;
                    position: relative;
                }
                h1 {
                    font-size: 36px;
                    margin-top: 50px;
                    margin-bottom: 20px;
                }
                h2 {
                    font-size: 18px;
                    margin-top: 0;
                }
                input[type='text'] {
                    padding: 10px;
                    border-radius: 5px;
                    border: none;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
                    margin-bottom: 20px;
                    width: 300px;
                    font-size: 16px;
                    background-color: #262626;
                    color: #f5f5f5;
                }
                input[type='text']:focus {
                    outline: none;
                    box-shadow: 0 0 10px rgba(255, 255, 255, 0.5);
                }
                button {
                    background-color: #007bff;
                    color: #fff;
                    padding: 10px 20px;
                    border: none;
                    border-radius: 5px;
                    font-size: 16px;
                    cursor: pointer;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
                }
                button:hover {
                    background-color: #0062cc;
                }
                .dark-mode {
                    background-color: #f5f5f5;
                    color: #1a1a1a;
                }
                .dark-mode h1,
                .dark-mode h2 {
                    color: #1a1a1a;
                }
                .dark-mode input[type='text'] {
                    background-color: #f5f5f5;
                    color: #1a1a1a;
                }
                .dark-mode input[type='text']:focus {
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
                }
                .dark-mode button {
                    background-color: #1a1a1a;
                    color: #f5f5f5;
                    box-shadow: 0 0 10px rgba(255, 255, 255, 0.2);
                }
                .dark-mode button:hover {
                    background-color: #262626;
                }
                .made-by {
                    position: absolute;
                    bottom: 10px;
                    right: 10px;
                    font-size: 12px;
                }
                .error-text {
                    color: #F00;
                    font-size: 12px;
                    margin-top: 4px;
                }
            </style>
            <script>
            function submit() {
                const token = document.getElementById('token').value;
              
                // check if token is empty
                if (token === '') {
                  const messageEl = document.getElementById('error');
                  messageEl.textContent = 'Please enter a token.';
                  messageEl.style.color = 'red';
                  return;
                }
              
                // call checkAccess() with the token
                checkAccess(token);
              }
              
              function checkAccess(token) {
                const url = `/api/access?key=${encodeURIComponent(token)}`;
              
                fetch(url, {
                  method: 'GET',
                  mode: 'no-cors',
                })
                  .then((response) => response.text())
                  .then((data) => {
                    if (data === 'access granted') {
                      // set cookie with expiration date of one day from now
                      const expirationDate = new Date();
                      expirationDate.setDate(expirationDate.getDate() + 1);
                      document.cookie = `access=${encodeURIComponent(token)}; expires=${expirationDate.toUTCString()}`;
                      window.location.href = '/dashboard';
                    } else {
                      // change text value
                      const messageEl = document.getElementById('error');
                      messageEl.textContent = 'Unknown token. Access denied.';
                      console.log('hey you, this is some sensitive things right here, pls dont bruteforce, you dont want to, i already have your ip, and all your moves are recorded...');
                    }
                  })
                  .catch((error) => {
                    console.error('Error fetching API:', error);
                  });
              }
            </script>
        </head>
        <body>
            <h1>Site Savior</h1>
            <h2>Your Swiss Army Knife for Managing Your Servers</h2>
            <input type='text' placeholder='Access Key' id='token' />
            <button onclick='submit()'>Go</button> <p></p>
            <span class='error-text' id='error'></span>
            <div class='made-by'>Made with ❤️ by Anass Zakar</div>
        </body>
    </html>".to_string()
}

pub fn dashboard() -> String {
    "<!DOCTYPE html>
    <html>
      <head>
        <meta charset='UTF-8' />
        <title>Dashboard</title>
        <style>
        body {
            background-color: #121212;
            color: #f7f7f7;
            font-family: 'Open Sans', sans-serif;
          }
          
          h1 {
            text-align: center;
            font-size: 48px;
            margin-top: 100px;
          }
          
          #dashboard {
            display: flex;
            justify-content: space-around;
            align-items: center;
            flex-wrap: wrap;
            margin-top: 50px;
          }
          
          .box {
            width: 300px;
            height: 200px;
            background-color: #262626;
            border-radius: 10px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: 36px;
            margin: 20px;
            transition: all 0.3s ease-in-out;
            cursor: pointer;
            align: center;
          }
          
          .box:hover {
            transform: scale(1.05);
          }
          
          .box:nth-child(1) {
            background-color: #393646;
          }
          
          .box:nth-child(2) {
            background-color: #4F4557;
          }
          
          .box:nth-child(3) {
            background-color: #6D5D6E;
          }
          
          .box:nth-child(4) {
            background-color: #D5CEA3;
          }
          
          #form {
            display: flex;
            justify-content: center;
            margin-top: 50px;
          }
          
          input[type='text'] {
            font-size: 24px;
            padding: 10px;
            border-radius: 5px;
            border: none;
            margin-right: 10px;
            width: 400px;
            background-color: #262626;
            color: #f7f7f7;
          }
          
          button[type='submit'] {
            background-color: #4caf50;
            color: #f7f7f7;
            font-size: 24px;
            padding: 10px;
            border-radius: 5px;
            border: none;
            cursor: pointer;
          }
          
          button[type='submit']:hover {
            background-color: #3e8e41;
          }

          #ddesc {
            font-size: 24px;
            color: #999;
            text-align: center;
            margin-top: 10px;
          }

          #noice {
            padding: 20px;
            border-radius: 10px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
            background-color: #fff;
            font-size: 24px;
            color: #3d3d3d;
            text-align: center;
            margin-top: 50px;
          }
        </style>
      </head>
      
      <body>
        <h1 id='dname'>Dashboard</h1>
        <p id='ddesc'>It's all what you need!</p>
        
        <div id='dashboard'>
          <div class='box' id='ram'>Box 1</div>
          <div class='box' id='disk_uage'>Box 2</div>
          <div class='box' id='cpu_freq'>Box 3</div>
          <div class='box' id='uptime'>Box 4</div>
        </div>
        
        <form id='form'>
          <input type='text' id='input' placeholder='Run Command On Machine...' />
          <button type='submit'>Go</button>
        </form>
        
        <p id='message'></p>
        
        <script>
        const form = document.getElementById('form');
        const input = document.getElementById('input');
        const message = document.getElementById('message');
        // hehe i love these js things, NaN != NaN, awesome, very logical
        const cookieValue = document.cookie
            .split('; ')
            .find(row => row.startsWith('access='))
            ?.split('=')[1];
        
        window.addEventListener('DOMContentLoaded', (event) => {
          fetch(`http://127.0.0.1:8000/api/info?token=${cookieValue}`)
          .then(response => response.json())
          .then(data => {
            document.getElementById('dname').textContent = data.name;
            document.getElementById('ddesc').textContent = data.desc;
            document.getElementById('ram').textContent = 'Ram:'+data.ram;
            document.getElementById('disk_uage').textContent = 'Disk:'+data.disk;
            document.getElementById('cpu_freq').textContent = 'Cpu Freq:'+data.cpu;
            document.getElementById('uptime').textContent = 'Uptime:'+data.uptime;
          })
          .catch(error => console.log(error));
        });
        
        form.addEventListener('submit', (event) => {
          event.preventDefault();
        
          const inputValue = input.value;
        
          if (inputValue === '') {
            message.textContent = 'Please enter a value';
            return;
          }
        
          fetch(`/exec?token=${encodeURIComponent(cookieValue)}&cmd=${encodeURIComponent(inputValue)}`)
            .then(response => {
              if (response.ok) {
                return response.text();
              } else {
                throw new Error('Network response was not OK');
              }
            })
            .then(data => {
              message.innerHTML = `<div id='noice'>${data}</div>`;
            })
            .catch(error => {
              console.error('Error fetching data from API:', error);
              message.textContent = 'Error fetching data from API';
            });
        });              
        </script>".to_string()
}