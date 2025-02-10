const fs = require('node:fs');

const file = fs.readFileSync('add-transaction-on-raspberry-with-stordy.txt', 'utf-8');

const transactions = file.split('\n').map((line) => {
  const [,time] = line.split('\t').filter((el) => el !== '');
  return time;
});

fs.writeFileSync('transactions.txt', transactions.join(', '));
