"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[470],{3905:(e,t,n)=>{n.d(t,{Zo:()=>u,kt:()=>h});var i=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);t&&(i=i.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,i)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,i,r=function(e,t){if(null==e)return{};var n,i,r={},o=Object.keys(e);for(i=0;i<o.length;i++)n=o[i],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(i=0;i<o.length;i++)n=o[i],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var s=i.createContext({}),c=function(e){var t=i.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},u=function(e){var t=c(e.components);return i.createElement(s.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return i.createElement(i.Fragment,{},t)}},d=i.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,s=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),d=c(n),h=r,m=d["".concat(s,".").concat(h)]||d[h]||p[h]||o;return n?i.createElement(m,a(a({ref:t},u),{},{components:n})):i.createElement(m,a({ref:t},u))}));function h(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,a=new Array(o);a[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:r,a[1]=l;for(var c=2;c<o;c++)a[c]=n[c];return i.createElement.apply(null,a)}return i.createElement.apply(null,n)}d.displayName="MDXCreateElement"},8546:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>a,default:()=>p,frontMatter:()=>o,metadata:()=>l,toc:()=>c});var i=n(7462),r=(n(7294),n(3905));const o={sidebar_position:2,title:"Building the ink! smart contract"},a=void 0,l={unversionedId:"tutorial/building",id:"tutorial/building",title:"Building the ink! smart contract",description:"To build the ink! smart contract we will need cargo-contract. So if we satisfy this condition, we will navigate to the generated folder ERC20 and call cargo contract build. The contract will start building, we will wait for a while and...",source:"@site/docs/tutorial/building.md",sourceDirName:"tutorial",slug:"/tutorial/building",permalink:"/tutorial/building",draft:!1,editUrl:"https://github.com/Supercolony-net/sol2ink/tree/main/docs/docs/tutorial/building.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2,title:"Building the ink! smart contract"},sidebar:"tutorialSidebar",previous:{title:"Preparation",permalink:"/tutorial/preparation"},next:{title:"Parsing",permalink:"/how_it_works/parsing"}},s={},c=[{value:"Warnings",id:"warnings",level:3},{value:"More things to notice",id:"more-things-to-notice",level:3}],u={toc:c};function p(e){let{components:t,...n}=e;return(0,r.kt)("wrapper",(0,i.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"To build the ink! smart contract we will need ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/paritytech/cargo-contract"},"cargo-contract"),". So if we satisfy this condition, we will navigate to the generated folder ",(0,r.kt)("inlineCode",{parentName:"p"},"ERC20")," and call cargo contract build. The contract will start building, we will wait for a while and..."),(0,r.kt)("p",null,"It fails! So let's look at the issue."),(0,r.kt)("p",null,"First issue looks like this:"),(0,r.kt)("p",null,(0,r.kt)("img",{parentName:"p",src:"https://user-images.githubusercontent.com/43150707/183226415-17fa4232-9b38-4302-b8b8-4357c64ab740.png",alt:"issue1"})),(0,r.kt)("p",null,"This issue was described before, Solidity expression is written as ",(0,r.kt)("inlineCode",{parentName:"p"},"type(uint256).max")," and gets parsed as ",(0,r.kt)("inlineCode",{parentName:"p"},"u128.max"),". The correct form is ",(0,r.kt)("inlineCode",{parentName:"p"},"u128::MAX"),", so now with no issues, we will try to build it again."),(0,r.kt)("p",null,"And we failed again."),(0,r.kt)("p",null,(0,r.kt)("img",{parentName:"p",src:"https://user-images.githubusercontent.com/43150707/183226417-73ec8940-0800-4e70-9bee-91421def1ba2.png",alt:"issue2"})),(0,r.kt)("p",null,"These issues have the same reason, we want to return String, which can not be copied. Fixing both of these issues is simple, we will return the clones of these strings by calling ",(0,r.kt)("inlineCode",{parentName:"p"},".clone()")," on them. Now when we build, everything works! Congratulations!"),(0,r.kt)("h3",{id:"warnings"},"Warnings"),(0,r.kt)("p",null,"You could have noticed some warnings. The cause of these warnings is that two of the functions have parameters, which are unused inside that function. This is not an issue, but if we want to remove these warnings, we will simply add ",(0,r.kt)("inlineCode",{parentName:"p"},"_")," to the front of the names of these parameters, which will imply that those parameters are unused. "),(0,r.kt)("h3",{id:"more-things-to-notice"},"More things to notice"),(0,r.kt)("p",null,"There are still some things, which are not implemented in Sol2Ink (but definitely on the radar!). Let's have a look at what was not parsed right in our ERC-20 file.\nFirst thing we notice is a comment on line 240 saying ",(0,r.kt)("inlineCode",{parentName:"p"},"Please handle unchecked blocks manually"),". Two lines above we see the same comment, but with inversed arrows, meaning that everything between these two comments is originally in the unchecked block. We don't really need to care about this, so we can just remove the comments. We can find the same comment on lines 270, 298, 331 and 392 where we do the same thing. And that's it! Now it is up to the developer to optimize the contract for Rust and ink!, but the dirty work is already done!"))}p.isMDXComponent=!0}}]);