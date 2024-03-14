/**
 * @author pxdraft
 * @version 1.0
 *
 */
(function ($) {
	'use strict';
	var CRE = {};
	$.fn.exists = function () {
		return this.length > 0;
	};

	CRE.UpdateCopyRight = function () {
		var yr = new Date().getFullYear();

		// update copyright date
		$('[class*="m-0 text-white-50 small text-center"]').html(
			['&copy; ', yr, ' Corporate Web Solutions Ltd. & WebAvail Productions Inc.'].join('')
		);
	};

	/*--------------------
      * Menu toogle header
    ----------------------*/
	CRE.MenuToggleClass = function () {
		$('.header-main .navbar-nav a').on('click', function () {
			var toggleA = $('.navbar-toggler').is(':visible');
			if (toggleA) {
				$('.navbar-collapse').collapse('hide');
			}
		});
		/*$('.navbar-toggler').on('click', function() {
            var toggle = $('.navbar-toggler').is(':visible');
            if (toggle) {
                $('header').addClass('header-toggle');
            }
        })*/
		$(document).click(function (e) {
			if (!$(e.target).is('.header-main .navbar-nav a')) {
				$('.header-main .collapse').collapse('hide');
			}
		});
	};

	/* ---------------------------------------------- /*
     * Header Fixed
    /* ---------------------------------------------- */
	CRE.HeaderFixd = function () {
		var HscrollTop = $(window).scrollTop();
		if (HscrollTop >= 65) {
			$('.navbar-dark').addClass('navbar-light');
			$('.navbar-dark').addClass('navbar-dark-top');
			$('.navbar-dark-top').removeClass('navbar-dark');
			$('.header-main').addClass('fixed-header');
		} else {
			$('.navbar-dark-top').removeClass('navbar-light');
			$('.navbar-dark-top').addClass('navbar-dark');
			$('.navbar-dark').removeClass('navbar-dark-top');
			$('.header-main').removeClass('fixed-header');
		}
	};

	/* ---------------------------------------------- /*
     * Header height
    /* ---------------------------------------------- */
	CRE.HeaderHeight = function () {
		var HHeight = $('.navbar').outerHeight();
		$('.header-height').css('min-height', HHeight);
		$('.header-height + .header-main').css('top', -HHeight);
	};

	/*--------------------
    * Owl Corousel
    ----------------------*/
	CRE.Owl = function () {
		var owlslider = $('div.owl-carousel');
		if (owlslider.length > 0) {
			owlslider.each(function () {
				var $this = $(this),
					$items = $this.data('items') ? $this.data('items') : 1,
					$loop = $this.attr('data-loop') ? $this.data('loop') : true,
					$navdots = $this.data('nav-dots') ? $this.data('nav-dots') : false,
					$navarrow = $this.data('nav-arrow') ? $this.data('nav-arrow') : false,
					$autoplay = $this.attr('data-autoplay') ? $this.data('autoplay') : true,
					$autospeed = $this.attr('data-autospeed') ? $this.data('autospeed') : 5000,
					$smartspeed = $this.attr('data-smartspeed') ? $this.data('smartspeed') : 1000,
					$autohgt = $this.data('autoheight') ? $this.data('autoheight') : false,
					$CenterSlider = $this.data('center') ? $this.data('center') : false,
					$stage = $this.attr('data-stage') ? $this.data('stage') : 0,
					$space = $this.attr('data-space') ? $this.data('space') : 30;

				$(this).owlCarousel({
					loop: $loop,
					items: $items,
					responsive: {
						0: {
							items: $this.data('xs-items') ? $this.data('xs-items') : 1
						},
						576: {
							items: $this.data('sm-items') ? $this.data('sm-items') : 1
						},
						768: {
							items: $this.data('md-items') ? $this.data('md-items') : 1
						},
						992: {
							items: $this.data('lg-items') ? $this.data('lg-items') : 1
						},
						1200: {
							items: $items
						}
					},
					dots: $navdots,
					autoplayTimeout: $autospeed,
					smartSpeed: $smartspeed,
					autoHeight: $autohgt,
					center: $CenterSlider,
					margin: $space,
					stagePadding: $stage,
					nav: $navarrow,
					navText: ["<i class='fas fa-chevron-left'></i>", "<i class='fas fa-chevron-right'></i>"],
					autoplay: $autoplay,
					autoplayHoverPause: true
				});
			});
		}
	};

	/*--------------------
        * Cookis Bar 
    ----------------------*/
	// CRE.Cookis = function () {
	// 	$(document).herbyCookie({
	// 		style: 'light',
	// 		btnText: 'Accept',
	// 		policyText: 'Privacy policy',
	// 		text:
	// 			'We use cookies to improve your browsing experience on our website, to analyze our website traffic, and to understand where our visitors are coming from. By browsing our website, you consent to our use of cookies.',
	// 		scroll: false,
	// 		expireDays: 30,
	// 		link: 'https://jscharting.com/policy.html'
	// 	});
	// };

	/*----------------------------------------
        * Doc Page Custom Scroll
    --------------------------------------------*/
	CRE.DocRightbar = function () {
		$('.doc-rightbar a[href^="#"]').on('click', function (e) {
			$('a').each(function () {
				$(this).removeClass('active');
			});
			$(this).addClass('active');
		});
	};

	// Window on Load
	$(window).on('load', function () {});
	// Document on Ready
	$(document).ready(function () {
		//CRE.Cookis();
		CRE.HeaderFixd();
		CRE.MenuToggleClass();
		CRE.HeaderHeight();
		CRE.DocRightbar();
		CRE.Owl();
		CRE.UpdateCopyRight();
	});

	// Document on Scrool
	$(window).scroll(function () {
		CRE.HeaderFixd();
	});

	// Window on Resize
	$(window).resize(function () {
		CRE.HeaderHeight();
	});
})(jQuery);
