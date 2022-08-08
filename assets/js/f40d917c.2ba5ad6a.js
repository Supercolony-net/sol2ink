"use strict";(self.webpackChunksol_2_ink=self.webpackChunksol_2_ink||[]).push([[57],{3905:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>h});var n=r(7294);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function i(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function a(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?i(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function l(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},i=Object.keys(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(n=0;n<i.length;n++)r=i[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var s=n.createContext({}),c=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):a(a({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(s.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},d=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,i=e.originalType,s=e.parentName,u=l(e,["components","mdxType","originalType","parentName"]),d=c(r),h=o,m=d["".concat(s,".").concat(h)]||d[h]||p[h]||i;return r?n.createElement(m,a(a({ref:t},u),{},{components:r})):n.createElement(m,a({ref:t},u))}));function h(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=r.length,a=new Array(i);a[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:o,a[1]=l;for(var c=2;c<i;c++)a[c]=r[c];return n.createElement.apply(null,a)}return n.createElement.apply(null,r)}d.displayName="MDXCreateElement"},5436:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>a,default:()=>p,frontMatter:()=>i,metadata:()=>l,toc:()=>c});var n=r(7462),o=(r(7294),r(3905));const i={sidebar_position:6,title:"Assembling a contract"},a=void 0,l={unversionedId:"how_it_works/assembler",id:"how_it_works/assembler",title:"Assembling a contract",description:"Sol2Ink has everything it needs, now it just needs to mix it together. Here we will just add some notes, which may not be obvious.",source:"@site/docs/how_it_works/assembler.md",sourceDirName:"how_it_works",slug:"/how_it_works/assembler",permalink:"/how_it_works/assembler",draft:!1,editUrl:"https://github.com/Supercolony-net/sol2ink/tree/main/docs/docs/how_it_works/assembler.md",tags:[],version:"current",sidebarPosition:6,frontMatter:{sidebar_position:6,title:"Assembling a contract"},sidebar:"tutorialSidebar",previous:{title:"Parsing expressions",permalink:"/how_it_works/parsing_expressions"},next:{title:"Known issues",permalink:"/issues"}},s={},c=[{value:"Error",id:"error",level:3},{value:"Storage",id:"storage",level:3}],u={toc:c};function p(e){let{components:t,...r}=e;return(0,o.kt)("wrapper",(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"Sol2Ink has everything it needs, now it just needs to mix it together. Here we will just add some notes, which may not be obvious."),(0,o.kt)("h3",{id:"error"},"Error"),(0,o.kt)("p",null,"Each contract will contain the following error definition: "),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'#[derive(Debug, Encode, Decode, PartialEq)]\n#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]\npub enum Error {\n    Custom(String),\n}\n')),(0,o.kt)("p",null,"This error will be used as the error type when returning results from the functions of the contract."),(0,o.kt)("h3",{id:"storage"},"Storage"),(0,o.kt)("p",null,"Openbrush simplifies the work with storage and allows the upgradeability of the storage, that is why we use the following approach. This approach will also simplify our future development, when our contract will be divided into multiple traits, etc. For now, a storage key is defined inside the contract, the state variables are defined in a struct which will use this storage key and this struct itself is the member of the contract storage. The whole storage will look something like this:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},"pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);\n\n#[derive(Default, Debug)]\n#[openbrush::upgradeable_storage(STORAGE_KEY)]\npub struct Data {\n    pub value: u128,\n}\n\n#[ink(storage)]\n#[derive(Default, SpreadAllocate, Storage)]\npub struct Contract {\n    #[storage_field]\n    data: Data,\n}\n")),(0,o.kt)("p",null,"Accessing the ",(0,o.kt)("inlineCode",{parentName:"p"},"value")," state variables inside the contract then looks like ",(0,o.kt)("inlineCode",{parentName:"p"},"self.data.value"),". "),(0,o.kt)("p",null,"The functions of the contract are then generated inside the impl section. Note the following:"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"the constructor will be called new, and will have the ",(0,o.kt)("inlineCode",{parentName:"li"},"#[ink(constructor)]")," attribute"),(0,o.kt)("li",{parentName:"ul"},"the constructor will be generated even if it is empty or does not exist in the original contract"),(0,o.kt)("li",{parentName:"ul"},"public/external messages will have the ",(0,o.kt)("inlineCode",{parentName:"li"},"#[ink(message)]")," attribute"),(0,o.kt)("li",{parentName:"ul"},"private/internal functions will be prefixed with ",(0,o.kt)("inlineCode",{parentName:"li"},"#_"))))}p.isMDXComponent=!0}}]);