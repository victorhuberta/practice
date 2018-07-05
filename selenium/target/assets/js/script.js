$(document).ready(() => {
  initImagePickers();

  $('.js-add-to-cart').click(() => {
    // Detach selected options from shop and append them to cart.
    $('.js-shop .image-picker option:selected')
      .detach()
      .appendTo('.js-cart .image-picker')
      .prop('selected', false);

    // Reinitialize image pickers.
    initImagePickers();
  });

  $('.js-remove-from-cart').click(() => {
    // Detach selected options from cart and append them to shop.
    $('.js-cart .image-picker option:selected')
      .detach()
      .appendTo('.js-shop .image-picker')
      .prop('selected', false);

    // Reinitialize image pickers.
    initImagePickers();
  });

  $('.js-checkout').click(() => {
    window.location.href = '/confirm-order.html';
  });

  $('.js-confirm-order').click(() => {
    // Inputs cannot be empty.
    if ($('.js-person-name').val().length === 0) { return; }
    if ($('.js-person-phone').val().length === 0) { return; }
    if ($('.js-person-address').val().length === 0) { return; }

    // Pretend to confirm order.
    window.alert('Your order has been confirmed!');
  });

  $('.js-submit-request').click(() => {
    // Inputs cannot be empty.
    if ($('.js-person-name').val().length === 0) { return; }
    if ($('.js-person-email').val().length === 0) { return; }
    if ($('.js-help-problem').val().length === 0) { return; }

    // Pretend to submit request.
    window.alert('Your help request has been submitted!');
  });
});

function initImagePickers() {
  const shopImagePicker = $('.js-shop .image-picker');
  if (shopImagePicker.length > 0) {
    shopImagePicker.imagepicker();
  }

  const cartImagePicker = $('.js-cart .image-picker');
  if (cartImagePicker.length > 0) {
    cartImagePicker.imagepicker();
  }
}
