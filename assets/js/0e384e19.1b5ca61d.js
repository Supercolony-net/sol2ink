"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[671],{3905:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>h});var o=n(7294);function i(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function r(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);t&&(o=o.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,o)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?r(Object(n),!0).forEach((function(t){i(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):r(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,o,i=function(e,t){if(null==e)return{};var n,o,i={},r=Object.keys(e);for(o=0;o<r.length;o++)n=r[o],t.indexOf(n)>=0||(i[n]=e[n]);return i}(e,t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);for(o=0;o<r.length;o++)n=r[o],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(i[n]=e[n])}return i}var s=o.createContext({}),c=function(e){var t=o.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},u=function(e){var t=c(e.components);return o.createElement(s.Provider,{value:t},e.children)},d={inlineCode:"code",wrapper:function(e){var t=e.children;return o.createElement(o.Fragment,{},t)}},p=o.forwardRef((function(e,t){var n=e.components,i=e.mdxType,r=e.originalType,s=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),p=c(n),h=i,m=p["".concat(s,".").concat(h)]||p[h]||d[h]||r;return n?o.createElement(m,a(a({ref:t},u),{},{components:n})):o.createElement(m,a({ref:t},u))}));function h(e,t){var n=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var r=n.length,a=new Array(r);a[0]=p;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:i,a[1]=l;for(var c=2;c<r;c++)a[c]=n[c];return o.createElement.apply(null,a)}return o.createElement.apply(null,n)}p.displayName="MDXCreateElement"},9881:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>a,default:()=>d,frontMatter:()=>r,metadata:()=>l,toc:()=>c});var o=n(7462),i=(n(7294),n(3905));const r={sidebar_position:1,slug:"/",title:"Sol2Ink Documentation",sidebar_label:"Getting started"},a="Sol2Ink Documentation",l={unversionedId:"intro",id:"intro",title:"Sol2Ink Documentation",description:"Welcome to Sol2Ink documentation. In this documentation, we will describe the capabilities of Sol2Ink, how the process works under the hood,",source:"@site/docs/intro.md",sourceDirName:".",slug:"/",permalink:"/",draft:!1,editUrl:"https://github.com/Supercolony-net/sol2ink/tree/main/docs/docs/intro.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1,slug:"/",title:"Sol2Ink Documentation",sidebar_label:"Getting started"},sidebar:"tutorialSidebar",next:{title:"Capabilities",permalink:"/capabilities"}},s={},c=[{value:"What is Sol2Ink",id:"what-is-sol2ink",level:2},{value:"What you&#39;ll need",id:"what-youll-need",level:3}],u={toc:c};function d(e){let{components:t,...n}=e;return(0,i.kt)("wrapper",(0,o.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"sol2ink-documentation"},"Sol2Ink Documentation"),(0,i.kt)("p",null,"Welcome to Sol2Ink documentation. In this documentation, we will describe the capabilities of Sol2Ink, how the process works under the hood,\nwhat issues you may face while using Sol2Ink, and you will see some examples of usage of Sol2Ink."),(0,i.kt)("h2",{id:"what-is-sol2ink"},"What is Sol2Ink"),(0,i.kt)("p",null,"Sol2Ink is a tool developed to ease the developers' transition from Solidity to ink!. Since we are the builders in the Dotsama ecosystem, we recognized a problem when some team wanted to develop their existing Solidity dapp in ink! smart contract language, the most annoying and time-consuming part of the development will be rewriting the Solidity code into Rust and ink!. Sol2Ink aims to decrease this time by transpiling the existing Solidity code into Rust and ink! code. So the dirty part of the job is automated, and now it is up to the developers to fix some language-specific issues while teaching how stuff works in ink!. Sol2Ink will save time!"),(0,i.kt)("h3",{id:"what-youll-need"},"What you'll need"),(0,i.kt)("p",null,"Sol2Ink is written in Rust, so that you will need Rust installed with the nightly toolchain. If this is satisfied, you will also need Sol2Ink, which you can get here. Another thing you will need is the Solidity file you want to transpile. And that's it! We can start transpiling now!"))}d.isMDXComponent=!0}}]);