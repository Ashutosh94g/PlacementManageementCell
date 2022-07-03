function Verify(email, password) {
    const validRegexEmail = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$/;
    const validRegexPassword = /^(?=.*\d)(?=.*[!@#$%^&*])(?=.*[a-z])(?=.*[A-Z]).{8,}$/;
    if(!email.match(validRegexEmail)) {
        alert("Please Enter a Valid Email Id");
        return false;
    }
    if(!password.match(validRegexPassword)) {
        alert("Please Enter a Password with atleast 1 uppercase 1 special character and 1 number and 8 characters long");
        return false;
    }
    
    return true;
}

export default Verify;