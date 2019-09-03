$(document).ready(function() {
    $('#loginForm').submit(function(e) {
        e.preventDefault();
        $('#warningMsg').hide();
        let username = $('#username').val().trim();
        let password = $('#password').val().trim();

        if(username.length > 0 && password.length > 0) {
            $.ajax({
                url: "/login",
                type: "POST",
                json: true,
                data: {
                    username,
                    password
                },
                success: function(data) {
                    window.location.assign('/rooms');
                },
                error: function() {
                    $('#warningMsg').text('Unable to login with provided username and password.');
                    $('#warningMsg').show();
                }
            })
        } else {
            $('#warningMsg').text('Please input a username and password.')
            $('#warningMsg').show()
        }
    })
})