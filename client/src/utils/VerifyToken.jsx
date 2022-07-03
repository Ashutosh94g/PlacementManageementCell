function VerifyToken(token){
    if(token.split(".").length === 3){
        return true;
    }
    return false;
}

export default VerifyToken;