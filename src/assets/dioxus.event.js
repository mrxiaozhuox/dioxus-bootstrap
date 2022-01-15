/**
 * Dioxus-Bootstrap Event JS
 * YuKun Liu <mrxzx.info@gmail.com>
 */

window.dioxus = {
    alert: function (placeholder, message, type = "primary") {
        $(placeholder).append(
            '<div class="alert alert-' + type
            + ' alert-dismissible fade show" role="alert">' + message
            + '<button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button></div>'
        );
    },
};