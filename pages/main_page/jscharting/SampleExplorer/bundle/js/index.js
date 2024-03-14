/*
* @license
* (c) 2011-2023 Corporate Web Solutions Ltd. This software may be used to support and facilitate your
* development utilizing JSCharting if you hold a valid license for JSCharting.  This software may not
* be used or integrated for any task independent of its JSCharting support role.
* Please see https://jscharting.com/store/ for JSCharting licensing options.
*
* */
/*
* @license
* (c) 2011-2023 Corporate Web Solutions Ltd. This software may be used to support and facilitate your
* development utilizing JSCharting if you hold a valid license for JSCharting.  This software may not
* be used or integrated for any task independent of its JSCharting support role.
* Please see https://jscharting.com/store/ for JSCharting licensing options.
*
* */
$(document).ready(function(){"undefined"!==typeof bundleMessage?"undefined"!==typeof JSC?bundleMessage.getMessage(JSC.getVersion()):$("#bundleMsg").html("/jsc/jscharting.js is not found. Please double check your installation or contact support for assistance."):"undefined"!==typeof JSC?$("#bundleMsg").html("JSCharting (v"+JSC.getVersion()+")"):$("#bundleMsg").html("/jsc/jscharting.js is not found. Please double check your installation or contact support for assistance.");"file:"!==window.location.protocol&&
$(".warning").hide()});$(document).ready(function(){$.ajax({url:"license.txt",dataType:"text",success:function(b){if(""!=b){makeLicenseNotice(b);var e=!0,g=$("#license-text");$(g).on("scroll",function(){$(g)[0].scrollHeight-$(g)[0].scrollTop<=$(g)[0].clientHeight+5&&e&&($("#license-modal .modal-footer button").css({display:"block"}),$("#license-modal .modal-footer p").css({display:"none"}),e=!1)})}}})});
function makeLicenseNotice(b){var e="JSC_notice_"+JSC.getVersion();$.cookie(e)||($('<div id="license-modal" class="modal fade" role="dialog" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true"><div class="modal-dialog modal-dialog-centered modal-lg" role="document"><div class="modal-content"><div class="modal-header"><h5 class="modal-title">JSCharting Licensing Agreement</h5></div><div class="modal-body"><textarea id="license-text" class="w-100" readonly></textarea></div><div class="modal-footer justify-content-center"><button id="decline-btn" type="button" class="btn btn-secondary mr-auto" data-dismiss="modal">Decline</button><button id="accept-btn" type="button" class="btn btn-primary mr-auto">Accept</button><p>Please read to the end of the agreement to Accept or Decline</p></div></div></div></div>').appendTo("body"),$("#license-modal .modal-footer").css({"min-height":"82px"}),
$("#license-modal .modal-footer button").css({display:"none"}),$("#license-text").css({height:"300px",resize:"none",border:"none"}),$("#license-text").html(b),$("#license-modal").modal("show"),$("#decline-btn").click(function(){$("#decline-btn").css({display:"none"});$("#license-modal .modal-footer p").css({display:"block"});$("#license-modal .modal-footer p").html('JSCharting may not be used without accepting the license terms, please contact <a href="mailto:orders@JSCharting.com">orders@JSCharting.com</a> for assistance.')}),
$("#accept-btn").click(function(){$("#license-modal").modal("hide");$.cookie(e,"true",{expires:365,path:"/"})}))}
(function(b){"function"===typeof define&&define.amd?define(["jquery"],b):b(jQuery)})(function(b){function e(a){return a}function g(a){return decodeURIComponent(a.replace(m," "))}function n(a){0===a.indexOf('"')&&(a=a.slice(1,-1).replace(/\\"/g,'"').replace(/\\\\/g,"\\"));try{return f.json?JSON.parse(a):a}catch(b){}}var m=/\+/g,f=b.cookie=function(a,d,c){if(void 0!==d){c=b.extend({},f.defaults,c);if("number"===typeof c.expires){var k=c.expires,h=c.expires=new Date;h.setDate(h.getDate()+k)}d=f.json?
JSON.stringify(d):String(d);return document.cookie=[f.raw?a:encodeURIComponent(a),"=",f.raw?d:encodeURIComponent(d),c.expires?"; expires="+c.expires.toUTCString():"",c.path?"; path="+c.path:"",c.domain?"; domain="+c.domain:"",c.secure?"; secure":""].join("")}d=f.raw?e:g;c=document.cookie.split("; ");for(var k=a?void 0:{},h=0,m=c.length;h<m;h++){var l=c[h].split("="),p=d(l.shift()),l=d(l.join("="));if(a&&a===p){k=n(l);break}a||(k[p]=n(l))}return k};f.defaults={};b.removeCookie=function(a,d){return void 0!==
b.cookie(a)?(b.cookie(a,"",b.extend({},d,{expires:-1})),!0):!1}});
