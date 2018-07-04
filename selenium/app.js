const express = require('express');
const app = express();

app.get('/', (req, res) => {
  res.redirect('/buy.html');
});

app.use(express.static(__dirname + '/target'));

app.listen(process.env.PORT || 8000, 'localhost', () => console.log('Running...'));
