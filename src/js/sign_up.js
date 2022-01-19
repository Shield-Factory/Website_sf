var password = document.getElementById("password");
var letter = document.getElementById("letter");
var capital = document.getElementById("capital");
var number = document.getElementById("number");
var special = document.getElementById("special");
var length = document.getElementById("length");
var password2 = document.getElementById("password2");



//When the user clicks on the password box, display the conditions
password.onfocus = function() {
  document.getElementById("message").style.display = "block";
}

//When the user clicks outside the password box, hide the message box
password.onblur = function() {
  document.getElementById("message").style.display = "none";
}

//When the user starts to type something on the boxe
password.onkeyup = function () {
  //validate lowercase letters
  var lowerCaseLetters = /[a-z]/g;
  if(password.value.match(lowerCaseLetters)) {
    letter.classList.remove("invalid");
    letter.classList.add("valid");
  } else {
    letter.classList.remove("valid");
    letter.classList.add("invalid");
  }

  //Validate capital letters
  var upperCaseLetters = /[A-Z]/g;
  if(password.value.match(upperCaseLetters)) {
    capital.classList.remove("invalid");
    capital.classList.add("valid");
  } else {
    capital.classList.remove("valid");
    capital.classList.add("invalid");
  }

  //Validate number
  var numbers = /[0-9]/g;
  if(password.value.match(numbers)) {
    number.classList.remove("invalid");
    number.classList.add("valid");
  } else {
    number.classList.remove("valid");
    number.classList.add("invalid");
  }

  //Validate special characters
  var specialCharacters = /[^A-Za-z0-9]/g;
  if(password.value.match(specialCharacters)) {
    special.classList.remove("invalid");
    special.classList.add("valid");
  } else {
    special.classList.remove("valid");
    special.classList.add("invalid");
  }

  //Validate length
  if(password.value.length >= 14) {
    length.classList.remove("invalid");
    length.classList.add("valid");
  } else {
    length.classList.remove("valid");
    length.classList.add("invalid");
  }
}

//Check if the 2 password match
function validatePassword() {
  if(password.value != password2.value) {
    password2.setCustomValidity("Passwords don't match");
  } else {
    password2.setCustomValidity("");
  }
}

password.onchange = validatePassword;
password2.onkeyup = validatePassword;
