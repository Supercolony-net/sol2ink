"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[615],{3905:(e,t,r)=>{r.d(t,{Zo:()=>c,kt:()=>d});var n=r(7294);function i(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function a(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){i(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,i=function(e,t){if(null==e)return{};var r,n,i={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(i[r]=e[r]);return i}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(i[r]=e[r])}return i}var s=n.createContext({}),p=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):a(a({},t),e)),r},c=function(e){var t=p(e.components);return n.createElement(s.Provider,{value:t},e.children)},u={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},f=n.forwardRef((function(e,t){var r=e.components,i=e.mdxType,o=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),f=p(r),d=i,m=f["".concat(s,".").concat(d)]||f[d]||u[d]||o;return r?n.createElement(m,a(a({ref:t},c),{},{components:r})):n.createElement(m,a({ref:t},c))}));function d(e,t){var r=arguments,i=t&&t.mdxType;if("string"==typeof e||i){var o=r.length,a=new Array(o);a[0]=f;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:i,a[1]=l;for(var p=2;p<o;p++)a[p]=r[p];return n.createElement.apply(null,a)}return n.createElement.apply(null,r)}f.displayName="MDXCreateElement"},7226:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>a,default:()=>u,frontMatter:()=>o,metadata:()=>l,toc:()=>p});var n=r(7462),i=(r(7294),r(3905));const o={sidebar_position:1,title:"Parsing"},a=void 0,l={unversionedId:"how_it_works/parsing",id:"how_it_works/parsing",title:"Parsing",description:"In this section we will take a look at how Sol2Ink works under the hood.",source:"@site/docs/how_it_works/parsing.md",sourceDirName:"how_it_works",slug:"/how_it_works/parsing",permalink:"/how_it_works/parsing",draft:!1,editUrl:"https://github.com/Supercolony-net/sol2ink/tree/main/docs/docs/how_it_works/parsing.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"Parsing"},sidebar:"tutorialSidebar",previous:{title:"Building the ink! smart contract",permalink:"/tutorial/building"},next:{title:"Parsing an interface",permalink:"/how_it_works/parsing_interface"}},s={},p=[{value:"Parsing",id:"parsing",level:3},{value:"Note the following",id:"note-the-following",level:3}],c={toc:p};function u(e){let{components:t,...r}=e;return(0,i.kt)("wrapper",(0,n.Z)({},c,r,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("p",null,"In this section we will take a look at how Sol2Ink works under the hood."),(0,i.kt)("h3",{id:"parsing"},"Parsing"),(0,i.kt)("p",null,"First thing the program will do after running it is parse the original file. In the first phase it will look for the contract or interface definition. It will also parse all comments which it finds until finding the definition of the contract. Imports and pragma statement are not needed in the ink! file (altough support for multi-file projects will be implemented in later versions), so they are skipped. Once we find the contract or interface definition, we start parsing it."),(0,i.kt)("h3",{id:"note-the-following"},"Note the following"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"library parsing is not implemented yet"),(0,i.kt)("li",{parentName:"ul"},"inheritance parsing is not implemented yet, so everything after ",(0,i.kt)("inlineCode",{parentName:"li"},"is")," keyword will be skipped"),(0,i.kt)("li",{parentName:"ul"},"if the parser fails to find contract or interface definition, it will fail")))}u.isMDXComponent=!0}}]);