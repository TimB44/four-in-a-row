var nt=Object.defineProperty;var rt=(e,t,n)=>t in e?nt(e,t,{enumerable:!0,configurable:!0,writable:!0,value:n}):e[t]=n;var we=(e,t,n)=>(rt(e,typeof t!="symbol"?t+"":t,n),n);(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const l of document.querySelectorAll('link[rel="modulepreload"]'))r(l);new MutationObserver(l=>{for(const o of l)if(o.type==="childList")for(const s of o.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&r(s)}).observe(document,{childList:!0,subtree:!0});function n(l){const o={};return l.integrity&&(o.integrity=l.integrity),l.referrerPolicy&&(o.referrerPolicy=l.referrerPolicy),l.crossOrigin==="use-credentials"?o.credentials="include":l.crossOrigin==="anonymous"?o.credentials="omit":o.credentials="same-origin",o}function r(l){if(l.ep)return;l.ep=!0;const o=n(l);fetch(l.href,o)}})();function I(){}const lt=e=>e;function Me(e,t){for(const n in t)e[n]=t[n];return e}function Ue(e){return e()}function je(){return Object.create(null)}function V(e){e.forEach(Ue)}function He(e){return typeof e=="function"}function H(e,t){return e!=e?t==t:e!==t||e&&typeof e=="object"||typeof e=="function"}function it(e){return Object.keys(e).length===0}function ot(e,...t){if(e==null){for(const r of t)r(void 0);return I}const n=e.subscribe(...t);return n.unsubscribe?()=>n.unsubscribe():n}function Pe(e,t,n){e.$$.on_destroy.push(ot(t,n))}const Je=typeof window<"u";let st=Je?()=>window.performance.now():()=>Date.now(),Ke=Je?e=>requestAnimationFrame(e):I;const ee=new Set;function Ve(e){ee.forEach(t=>{t.c(e)||(ee.delete(t),t.f())}),ee.size!==0&&Ke(Ve)}function ct(e){let t;return ee.size===0&&Ke(Ve),{promise:new Promise(n=>{ee.add(t={c:e,f:n})}),abort(){ee.delete(t)}}}function g(e,t){e.appendChild(t)}function M(e,t,n){e.insertBefore(t,n||null)}function O(e){e.parentNode&&e.parentNode.removeChild(e)}function k(e){return document.createElement(e)}function z(e){return document.createElementNS("http://www.w3.org/2000/svg",e)}function W(e){return document.createTextNode(e)}function N(){return W(" ")}function ut(){return W("")}function J(e,t,n,r){return e.addEventListener(t,n,r),()=>e.removeEventListener(t,n,r)}function _(e,t,n){n==null?e.removeAttribute(t):e.getAttribute(t)!==n&&e.setAttribute(t,n)}function $e(e){let t;return{p(...n){t=n,t.forEach(r=>e.push(r))},r(){t.forEach(n=>e.splice(e.indexOf(n),1))}}}function ft(e){return Array.from(e.childNodes)}function at(e,t){t=""+t,e.data!==t&&(e.data=t)}function le(e,t){e.value=t??""}function dt(e,t,{bubbles:n=!1,cancelable:r=!1}={}){return new CustomEvent(e,{detail:t,bubbles:n,cancelable:r})}let oe;function ie(e){oe=e}function Qe(){if(!oe)throw new Error("Function called outside component initialization");return oe}function ht(e){Qe().$$.on_mount.push(e)}function he(){const e=Qe();return(t,n,{cancelable:r=!1}={})=>{const l=e.$$.callbacks[t];if(l){const o=dt(t,n,{cancelable:r});return l.slice().forEach(s=>{s.call(e,o)}),!o.defaultPrevented}return!0}}const x=[],G=[];let te=[];const Ne=[],pt=Promise.resolve();let ve=!1;function _t(){ve||(ve=!0,pt.then(Xe))}function Ee(e){te.push(e)}const ke=new Set;let Y=0;function Xe(){if(Y!==0)return;const e=oe;do{try{for(;Y<x.length;){const t=x[Y];Y++,ie(t),gt(t.$$)}}catch(t){throw x.length=0,Y=0,t}for(ie(null),x.length=0,Y=0;G.length;)G.pop()();for(let t=0;t<te.length;t+=1){const n=te[t];ke.has(n)||(ke.add(n),n())}te.length=0}while(x.length);for(;Ne.length;)Ne.pop()();ve=!1,ke.clear(),ie(e)}function gt(e){if(e.fragment!==null){e.update(),V(e.before_update);const t=e.dirty;e.dirty=[-1],e.fragment&&e.fragment.p(e.ctx,t),e.after_update.forEach(Ee)}}function mt(e){const t=[],n=[];te.forEach(r=>e.indexOf(r)===-1?t.push(r):n.push(r)),n.forEach(r=>r()),te=t}const fe=new Set;let K;function Ce(){K={r:0,c:[],p:K}}function Se(){K.r||V(K.c),K=K.p}function j(e,t){e&&e.i&&(fe.delete(e),e.i(t))}function T(e,t,n,r){if(e&&e.o){if(fe.has(e))return;fe.add(e),K.c.push(()=>{fe.delete(e),r&&(n&&e.d(1),r())}),e.o(t)}else r&&r()}function ae(e){return(e==null?void 0:e.length)!==void 0?e:Array.from(e)}function bt(e,t){e.d(1),t.delete(e.key)}function yt(e,t){T(e,1,1,()=>{t.delete(e.key)})}function Ye(e,t,n,r,l,o,s,u,i,c,p,f){let a=e.length,y=o.length,d=a;const m={};for(;d--;)m[e[d].key]=d;const h=[],C=new Map,w=new Map,D=[];for(d=y;d--;){const v=f(l,o,d),A=n(v);let $=s.get(A);$?r&&D.push(()=>$.p(v,t)):($=c(A,v),$.c()),C.set(A,h[d]=$),A in m&&w.set(A,Math.abs(d-m[A]))}const S=new Set,b=new Set;function E(v){j(v,1),v.m(u,p),s.set(v.key,v),p=v.first,y--}for(;a&&y;){const v=h[y-1],A=e[a-1],$=v.key,L=A.key;v===A?(p=v.first,a--,y--):C.has(L)?!s.has($)||S.has($)?E(v):b.has(L)?a--:w.get($)>w.get(L)?(b.add($),E(v)):(S.add(L),a--):(i(A,s),a--)}for(;a--;){const v=e[a];C.has(v.key)||i(v,s)}for(;y;)E(h[y-1]);return V(D),h}function U(e){e&&e.c()}function R(e,t,n){const{fragment:r,after_update:l}=e.$$;r&&r.m(t,n),Ee(()=>{const o=e.$$.on_mount.map(Ue).filter(He);e.$$.on_destroy?e.$$.on_destroy.push(...o):V(o),e.$$.on_mount=[]}),l.forEach(Ee)}function q(e,t){const n=e.$$;n.fragment!==null&&(mt(n.after_update),V(n.on_destroy),n.fragment&&n.fragment.d(t),n.on_destroy=n.fragment=null,n.ctx=[])}function wt(e,t){e.$$.dirty[0]===-1&&(x.push(e),_t(),e.$$.dirty.fill(0)),e.$$.dirty[t/31|0]|=1<<t%31}function Q(e,t,n,r,l,o,s=null,u=[-1]){const i=oe;ie(e);const c=e.$$={fragment:null,ctx:[],props:o,update:I,not_equal:l,bound:je(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(t.context||(i?i.$$.context:[])),callbacks:je(),dirty:u,skip_bound:!1,root:t.target||i.$$.root};s&&s(c.root);let p=!1;if(c.ctx=n?n(e,t.props||{},(f,a,...y)=>{const d=y.length?y[0]:a;return c.ctx&&l(c.ctx[f],c.ctx[f]=d)&&(!c.skip_bound&&c.bound[f]&&c.bound[f](d),p&&wt(e,f)),a}):[],c.update(),p=!0,V(c.before_update),c.fragment=r?r(c.ctx):!1,t.target){if(t.hydrate){const f=ft(t.target);c.fragment&&c.fragment.l(f),f.forEach(O)}else c.fragment&&c.fragment.c();t.intro&&j(e.$$.fragment),R(e,t.target,t.anchor),Xe()}ie(i)}class X{constructor(){we(this,"$$");we(this,"$$set")}$destroy(){q(this,1),this.$destroy=I}$on(t,n){if(!He(n))return I;const r=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return r.push(n),()=>{const l=r.indexOf(n);l!==-1&&r.splice(l,1)}}$set(t){this.$$set&&!it(t)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}const $t="4";typeof window<"u"&&(window.__svelte||(window.__svelte={v:new Set})).v.add($t);function Ze(e){let t=kt(e);return t!==0||(t=vt(e),t!==0)||(t=Et(e),t!==0)||(t=Ct(e)),t}function kt(e){for(let t=0;t<3;t++)e:for(let n=0;n<7;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t+l][n]!==r)continue e;return r}}return 0}function vt(e){for(let t=0;t<6;t++)e:for(let n=0;n<4;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t][n+l]!==r)continue e;return r}}return 0}function Et(e){for(let t=0;t<3;t++)e:for(let n=0;n<4;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t+l][n+l]!==r)continue e;return r}}return 0}function Ct(e){for(let t=3;t<6;t++)e:for(let n=0;n<4;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t-l][n+l]!==r)continue e;return r}}return 0}const Z=[];function xe(e,t=I){let n;const r=new Set;function l(u){if(H(e,u)&&(e=u,n)){const i=!Z.length;for(const c of r)c[1](),Z.push(c,e);if(i){for(let c=0;c<Z.length;c+=2)Z[c][0](Z[c+1]);Z.length=0}}}function o(u){l(u(e))}function s(u,i=I){const c=[u,i];return r.add(c),r.size===1&&(n=t(l,o)||I),u(e),()=>{r.delete(c),r.size===0&&n&&(n(),n=null)}}return{set:l,update:o,subscribe:s}}function Le(e){return Object.prototype.toString.call(e)==="[object Date]"}function St(e){const t=e-1;return t*t*t+1}function Ae(e,t){if(e===t||e!==e)return()=>e;const n=typeof e;if(n!==typeof t||Array.isArray(e)!==Array.isArray(t))throw new Error("Cannot interpolate values of different type");if(Array.isArray(e)){const r=t.map((l,o)=>Ae(e[o],l));return l=>r.map(o=>o(l))}if(n==="object"){if(!e||!t)throw new Error("Object cannot be null");if(Le(e)&&Le(t)){e=e.getTime(),t=t.getTime();const o=t-e;return s=>new Date(e+s*o)}const r=Object.keys(t),l={};return r.forEach(o=>{l[o]=Ae(e[o],t[o])}),o=>{const s={};return r.forEach(u=>{s[u]=l[u](o)}),s}}if(n==="number"){const r=t-e;return l=>e+l*r}throw new Error(`Cannot interpolate ${n} values`)}function At(e,t={}){const n=xe(e);let r,l=e;function o(s,u){if(e==null)return n.set(e=s),Promise.resolve();l=s;let i=r,c=!1,{delay:p=0,duration:f=400,easing:a=lt,interpolate:y=Ae}=Me(Me({},t),u);if(f===0)return i&&(i.abort(),i=null),n.set(e=l),Promise.resolve();const d=st()+p;let m;return r=ct(h=>{if(h<d)return!0;c||(m=y(e,s),typeof f=="function"&&(f=f(e,s)),c=!0),i&&(i.abort(),i=null);const C=h-d;return C>f?(n.set(e=s),!1):(n.set(e=m(a(C/f))),!0)}),r.promise}return{set:o,update:(s,u)=>o(s(l,e),u),subscribe:n.subscribe}}function Pt(e){let t,n;return{c(){t=z("circle"),_(t,"cx",e[2]),_(t,"cy",e[1]),_(t,"r","45"),_(t,"fill",n=e[0]==="red"?"#E53734":"#04BEBE")},m(r,l){M(r,t,l)},p(r,[l]){l&2&&_(t,"cy",r[1]),l&1&&n!==(n=r[0]==="red"?"#E53734":"#04BEBE")&&_(t,"fill",n)},i:I,o:I,d(r){r&&O(t)}}}function Dt(e,t,n){let r,{row:l}=t,{col:o}=t,{color:s}=t,u=50+100*o,i=At(50,{delay:50,easing:St,duration:(c,p)=>Math.sqrt(Math.abs(c-p)*400)});return Pe(e,i,c=>n(1,r=c)),i.set(150+100*(5-l)),e.$$set=c=>{"row"in c&&n(4,l=c.row),"col"in c&&n(5,o=c.col),"color"in c&&n(0,s=c.color)},[s,r,u,i,l,o]}class Ot extends X{constructor(t){super(),Q(this,t,Dt,Pt,H,{row:4,col:5,color:0})}}function Te(e,t,n){const r=e.slice();return r[3]=t[n].row,r[4]=t[n].col,r[5]=t[n].color,r[7]=n,r}function ze(e,t){let n,r,l;return r=new Ot({props:{row:t[3],col:t[4],color:t[5]}}),{key:e,first:null,c(){n=ut(),U(r.$$.fragment),this.first=n},m(o,s){M(o,n,s),R(r,o,s),l=!0},p(o,s){t=o;const u={};s&1&&(u.row=t[3]),s&1&&(u.col=t[4]),s&1&&(u.color=t[5]),r.$set(u)},i(o){l||(j(r.$$.fragment,o),l=!0)},o(o){T(r.$$.fragment,o),l=!1},d(o){o&&O(n),q(r,o)}}}function It(e){let t,n,r,l,o,s,u,i=[],c=new Map,p,f,a=ae(e[0]);const y=d=>d[7];for(let d=0;d<a.length;d+=1){let m=Te(e,a,d),h=y(m);c.set(h,i[d]=ze(h,m))}return{c(){t=z("svg"),n=z("defs"),r=z("pattern"),l=z("circle"),o=z("mask"),s=z("rect"),u=z("rect");for(let d=0;d<i.length;d+=1)i[d].c();p=z("rect"),_(l,"cx","50"),_(l,"cy","50"),_(l,"r","45"),_(l,"fill","black"),_(r,"id","cell-pattern"),_(r,"patternUnits","userSpaceOnUse"),_(r,"width","100"),_(r,"height","100"),_(s,"width","700"),_(s,"height","700"),_(s,"fill","white"),_(u,"width","700"),_(u,"height","700"),_(u,"fill","url(#cell-pattern)"),_(o,"id","cell-mask"),_(p,"x","0"),_(p,"y","100"),_(p,"height","600"),_(p,"width","700"),_(p,"fill","#303030"),_(p,"mask","url(#cell-mask)"),_(t,"width","350px"),_(t,"viewBox","0 0 700 700"),_(t,"xmlns","http://www.w3.org/2000/svg"),_(t,"class","svelte-16b1qih")},m(d,m){M(d,t,m),g(t,n),g(n,r),g(r,l),g(n,o),g(o,s),g(o,u);for(let h=0;h<i.length;h+=1)i[h]&&i[h].m(t,null);g(t,p),f=!0},p(d,[m]){m&1&&(a=ae(d[0]),Ce(),i=Ye(i,m,y,1,d,a,c,t,yt,ze,p,Te),Se())},i(d){if(!f){for(let m=0;m<a.length;m+=1)j(i[m]);f=!0}},o(d){for(let m=0;m<i.length;m+=1)T(i[m]);f=!1},d(d){d&&O(t);for(let m=0;m<i.length;m+=1)i[m].d()}}}function Bt(e,t,n){let r=[];function l(s,u,i){n(0,r=[...r,{row:s,col:u,color:i}])}function o(){n(0,r=[])}return[r,l,o]}class et extends X{constructor(t){super(),Q(this,t,Bt,It,H,{placePiece:1,clear:2})}get placePiece(){return this.$$.ctx[1]}get clear(){return this.$$.ctx[2]}}function Ge(e,t,n){const r=e.slice();return r[8]=t[n],r[9]=t,r[10]=n,r}function Re(e,t){let n,r=t[10],l,o;const s=()=>t[5](n,r),u=()=>t[5](null,r);function i(){return t[6](t[10])}return{key:e,first:null,c(){n=k("button"),_(n,"class","svelte-1hizh5c"),this.first=n},m(c,p){M(c,n,p),s(),l||(o=J(n,"click",i),l=!0)},p(c,p){t=c,r!==t[10]&&(u(),r=t[10],s())},d(c){c&&O(n),u(),l=!1,o()}}}function Ft(e){let t,n=[],r=new Map,l=ae(Array(7));const o=s=>s[10];for(let s=0;s<l.length;s+=1){let u=Ge(e,l,s),i=o(u);r.set(i,n[s]=Re(i,u))}return{c(){t=k("div");for(let s=0;s<n.length;s+=1)n[s].c()},m(s,u){M(s,t,u);for(let i=0;i<n.length;i+=1)n[i]&&n[i].m(t,null)},p(s,[u]){u&3&&(l=ae(Array(7)),n=Ye(n,u,o,1,s,l,r,t,bt,Re,null,Ge))},i:I,o:I,d(s){s&&O(t);for(let u=0;u<n.length;u+=1)n[u].d()}}}function Mt(e,t,n){const r=he();let l=[];function o(){l.forEach(f=>f.disabled=!0)}function s(){l.forEach(f=>f.disabled=!1)}function u(f,a){f<0||f>6||n(0,l[f].disabled=a,l)}function i(f){f<0||f>6||r("buttonClick",{col:f})}function c(f,a){G[f?"unshift":"push"](()=>{l[a]=f,n(0,l)})}return[l,i,o,s,u,c,f=>i(f)]}class tt extends X{constructor(t){super(),Q(this,t,Mt,Ft,H,{disable:2,enable:3,setCol:4})}get disable(){return this.$$.ctx[2]}get enable(){return this.$$.ctx[3]}get setCol(){return this.$$.ctx[4]}}function jt(e){let t,n,r,l,o,s={};n=new et({props:s}),e[6](n);let u={};return l=new tt({props:u}),e[7](l),l.$on("buttonClick",e[8]),{c(){t=k("div"),U(n.$$.fragment),r=N(),U(l.$$.fragment)},m(i,c){M(i,t,c),R(n,t,null),g(t,r),R(l,t,null),o=!0},p(i,[c]){const p={};n.$set(p);const f={};l.$set(f)},i(i){o||(j(n.$$.fragment,i),j(l.$$.fragment,i),o=!0)},o(i){T(n.$$.fragment,i),T(l.$$.fragment,i),o=!1},d(i){i&&O(t),e[6](null),q(n),e[7](null),q(l)}}}function Nt(e,t,n){const r=he();let l=Array(7).fill(!1),o=Array.from({length:6},()=>Array(7).fill(0)),s=0,u,i;function c(w){r("gameEnd",{message:w,error:!1})}function p(w){for(let D=0;D<o.length;D++)if(o[D][w]===0)return D;return 6}function f(w){for(let E=0;E<7;E++)u.setCol(E,!0);let D=p(w);D===5&&(l[w]=!0);let S=s%2===0?1:-1;o[D][w]=S,i.placePiece(D,w,S===1?"red":"blue"),s++;let b=Ze(o);if(b!==0){c(`${b==1?"Red":"Blue"} Wins!`);return}s==42&&c("Draw");for(let E=0;E<7;E++)u.setCol(E,l[E])}function a(){y()}function y(){l.fill(!1),o.forEach(w=>w.fill(0)),s=0,i.clear(),u.enable()}function d(){a()}function m(w){G[w?"unshift":"push"](()=>{i=w,n(1,i)})}function h(w){G[w?"unshift":"push"](()=>{u=w,n(0,u)})}return[u,i,f,a,y,d,m,h,w=>{f(w.detail.col)}]}class Lt extends X{constructor(t){super(),Q(this,t,Nt,jt,H,{start:3,clear:4,restart:5})}get start(){return this.$$.ctx[3]}get clear(){return this.$$.ctx[4]}get restart(){return this.$$.ctx[5]}}const de=xe({mode:1,botSettings:{botDiff:"easy",playerIsFirst:!0}});function Tt(e){let t,n,r,l,o,s,u,i,c,p,f,a,y,d,m,h,C,w,D,S,b,E,v,A,$,L,De,Oe,ne,pe,Ie,se,B,ce,ue,Be,_e,re,ge,me,be,ye,Fe;return ge=$e(e[5][0]),me=$e(e[5][1]),be=$e(e[5][2]),{c(){t=k("h2"),t.textContent="Game Options",n=N(),r=k("div"),l=k("div"),o=k("h3"),o.textContent="Player 1 Color",s=N(),u=k("label"),i=k("input"),c=W(`
      Red`),p=N(),f=k("label"),a=k("input"),d=W(`
      Blue`),m=N(),h=k("div"),C=k("h3"),C.textContent="Number Of Players",w=N(),D=k("label"),S=k("input"),E=W(`
      One Player`),v=N(),A=k("label"),$=k("input"),De=W(`
      Two Player`),Oe=N(),ne=k("div"),pe=k("h3"),pe.textContent="AI Difficulty",Ie=N(),se=k("label"),B=k("input"),Be=W(`
      Easy`),_e=N(),re=k("button"),re.textContent="Start",i.checked=e[2],_(i,"name","color"),_(i,"type","radio"),i.__value=!0,le(i,i.__value),a.checked=y=!e[2],_(a,"name","color"),_(a,"type","radio"),a.__value=!1,le(a,a.__value),S.checked=b=e[0]==2,_(S,"name","playerNum"),_(S,"type","radio"),S.__value=2,le(S,S.__value),$.checked=L=e[0]==1,_($,"name","playerNum"),_($,"type","radio"),$.__value=1,le($,$.__value),B.checked=ce=e[1]==="easy",_(B,"name","AiDifficulty"),_(B,"type","radio"),B.__value="easy",le(B,B.__value),B.disabled=ue=e[0]!==2,_(r,"class","container svelte-hvlkr9"),ge.p(B),me.p(S,$),be.p(i,a)},m(P,F){M(P,t,F),M(P,n,F),M(P,r,F),g(r,l),g(l,o),g(l,s),g(l,u),g(u,i),i.checked=i.__value===e[2],g(u,c),g(l,p),g(l,f),g(f,a),a.checked=a.__value===e[2],g(f,d),g(r,m),g(r,h),g(h,C),g(h,w),g(h,D),g(D,S),S.checked=S.__value===e[0],g(D,E),g(h,v),g(h,A),g(A,$),$.checked=$.__value===e[0],g(A,De),g(r,Oe),g(r,ne),g(ne,pe),g(ne,Ie),g(ne,se),g(se,B),B.checked=B.__value===e[1],g(se,Be),M(P,_e,F),M(P,re,F),ye||(Fe=[J(i,"change",e[4]),J(a,"change",e[6]),J(S,"change",e[7]),J($,"change",e[8]),J(B,"change",e[9]),J(re,"click",e[10])],ye=!0)},p(P,[F]){F&4&&(i.checked=P[2]),F&4&&(i.checked=i.__value===P[2]),F&4&&y!==(y=!P[2])&&(a.checked=y),F&4&&(a.checked=a.__value===P[2]),F&1&&b!==(b=P[0]==2)&&(S.checked=b),F&1&&(S.checked=S.__value===P[0]),F&1&&L!==(L=P[0]==1)&&($.checked=L),F&1&&($.checked=$.__value===P[0]),F&2&&ce!==(ce=P[1]==="easy")&&(B.checked=ce),F&1&&ue!==(ue=P[0]!==2)&&(B.disabled=ue),F&2&&(B.checked=B.__value===P[1])},i:I,o:I,d(P){P&&(O(t),O(n),O(r),O(_e),O(re)),ge.r(),me.r(),be.r(),ye=!1,V(Fe)}}}function zt(e,t,n){let r;Pe(e,de,m=>n(11,r=m));let l=he(),o=r.mode,s=r.botSettings.botDiff,u=r.botSettings.playerIsFirst;const i=[[],[],[]];function c(){u=this.__value,n(2,u)}function p(){u=this.__value,n(2,u)}function f(){o=this.__value,n(0,o)}function a(){o=this.__value,n(0,o)}function y(){s=this.__value,n(1,s)}const d=m=>{l("gameStart",{})};return e.$$.update=()=>{e.$$.dirty&7&&de.set({mode:o,botSettings:{botDiff:s,playerIsFirst:u}})},[o,s,u,l,c,i,p,f,a,y,d]}class Gt extends X{constructor(t){super(),Q(this,t,zt,Tt,H,{})}}function Rt(e){let t,n,r,l,o,s={};n=new et({props:s}),e[8](n);let u={};return l=new tt({props:u}),e[9](l),l.$on("buttonClick",e[10]),{c(){t=k("div"),U(n.$$.fragment),r=N(),U(l.$$.fragment)},m(i,c){M(i,t,c),R(n,t,null),g(t,r),R(l,t,null),o=!0},p(i,[c]){const p={};n.$set(p);const f={};l.$set(f)},i(i){o||(j(n.$$.fragment,i),j(l.$$.fragment,i),o=!0)},o(i){T(n.$$.fragment,i),T(l.$$.fragment,i),o=!1},d(i){i&&O(t),e[8](null),q(n),e[9](null),q(l)}}}function qt(e,t,n){let{botDiff:r="easy"}=t,{playerIsFirst:l=!0}=t;const o=he();let s=Array(7).fill(!1),u=Array.from({length:6},()=>Array(7).fill(0)),i=0,c,p;console.log("in bot game control."),console.log(`player is first: ${l}`),ht(()=>{c.disable()});function f(b){o("gameEnd",{message:b,error:!1})}function a(b){for(let E=0;E<u.length;E++)if(u[E][b]===0)return E;return 6}function y(b){c.disable();let E=a(b);E===5&&(s[b]=!0);let v=i%2===0&&l||i%2===1&&!l?1:-1;u[E][b]=v,p.placePiece(E,b,v===1?"red":"blue"),i++;let A=Ze(u);if(A!==0){f(`${A==1?"Red":"Blue"} Wins!`);return}if(i==42){f("Draw");return}v==1?d():s.forEach(($,L)=>c.setCol(L,$))}async function d(){let b=Date.now(),$=(await(await fetch("/ai",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({board:u,difficulty:r})})).json()).move,L=Date.now()-b;L<250?setTimeout(()=>{y($)},250-L):y($)}function m(){l?c.enable():(c.disable(),d())}function h(){s.fill(!1),u.forEach(b=>b.fill(0)),i=0,p.clear(),c.disable()}function C(){h(),m()}function w(b){G[b?"unshift":"push"](()=>{p=b,n(1,p)})}function D(b){G[b?"unshift":"push"](()=>{c=b,n(0,c)})}const S=b=>{y(b.detail.col)};return e.$$set=b=>{"botDiff"in b&&n(3,r=b.botDiff),"playerIsFirst"in b&&n(4,l=b.playerIsFirst)},[c,p,y,r,l,m,h,C,w,D,S]}class Wt extends X{constructor(t){super(),Q(this,t,qt,Rt,H,{botDiff:3,playerIsFirst:4,start:5,clear:6,restart:7})}get start(){return this.$$.ctx[5]}get clear(){return this.$$.ctx[6]}get restart(){return this.$$.ctx[7]}}function qe(e){let t,n;return t=new Gt({}),t.$on("gameStart",e[6]),{c(){U(t.$$.fragment)},m(r,l){R(t,r,l),n=!0},p:I,i(r){n||(j(t.$$.fragment,r),n=!0)},o(r){T(t.$$.fragment,r),n=!1},d(r){q(t,r)}}}function We(e){let t,n;return{c(){t=k("p"),n=W(e[1])},m(r,l){M(r,t,l),g(t,n)},p(r,l){l&2&&at(n,r[1])},d(r){r&&O(t)}}}function Ut(e){let t;return{c(){t=k("p"),t.textContent="Error Please Refresh Page"},m(n,r){M(n,t,r)},p:I,i:I,o:I,d(n){n&&O(t)}}}function Ht(e){let t,n,r={botDiff:e[4].botSettings.botDiff,playerIsFirst:e[4].botSettings.playerIsFirst};return t=new Wt({props:r}),e[8](t),t.$on("gameEnd",e[5]),{c(){U(t.$$.fragment)},m(l,o){R(t,l,o),n=!0},p(l,o){const s={};o&16&&(s.botDiff=l[4].botSettings.botDiff),o&16&&(s.playerIsFirst=l[4].botSettings.playerIsFirst),t.$set(s)},i(l){n||(j(t.$$.fragment,l),n=!0)},o(l){T(t.$$.fragment,l),n=!1},d(l){e[8](null),q(t,l)}}}function Jt(e){let t,n,r={};return t=new Lt({props:r}),e[7](t),t.$on("gameEnd",e[5]),{c(){U(t.$$.fragment)},m(l,o){R(t,l,o),n=!0},p(l,o){const s={};t.$set(s)},i(l){n||(j(t.$$.fragment,l),n=!0)},o(l){T(t.$$.fragment,l),n=!1},d(l){e[7](null),q(t,l)}}}function Kt(e){let t,n,r,l,o,s,u,i,c,p,f=!e[2]&&qe(e),a=e[1]!==""&&We(e);const y=[Jt,Ht,Ut],d=[];function m(h,C){return h[0]===1?0:h[0]==2?1:2}return i=m(e),c=d[i]=y[i](e),{c(){t=k("head"),t.innerHTML="<title>Four In a Row</title>",n=N(),r=k("main"),l=k("h1"),l.textContent="Four In a Row",o=N(),f&&f.c(),s=N(),a&&a.c(),u=N(),c.c(),_(r,"class","svelte-1x652mb")},m(h,C){M(h,t,C),M(h,n,C),M(h,r,C),g(r,l),g(r,o),f&&f.m(r,null),g(r,s),a&&a.m(r,null),g(r,u),d[i].m(r,null),p=!0},p(h,[C]){h[2]?f&&(Ce(),T(f,1,1,()=>{f=null}),Se()):f?(f.p(h,C),C&4&&j(f,1)):(f=qe(h),f.c(),j(f,1),f.m(r,s)),h[1]!==""?a?a.p(h,C):(a=We(h),a.c(),a.m(r,u)):a&&(a.d(1),a=null);let w=i;i=m(h),i===w?d[i].p(h,C):(Ce(),T(d[w],1,1,()=>{d[w]=null}),Se(),c=d[i],c?c.p(h,C):(c=d[i]=y[i](h),c.c()),j(c,1),c.m(r,null))},i(h){p||(j(f),j(c),p=!0)},o(h){T(f),T(c),p=!1},d(h){h&&(O(t),O(n),O(r)),f&&f.d(),a&&a.d(),d[i].d()}}}function Vt(e,t,n){let r;Pe(e,de,a=>n(4,r=a));let l=!1,o;de.subscribe(a=>n(0,o=a.mode));let s="",u;function i(a){a.detail.error?n(1,s=`Error: ${a.detail.message}`):n(1,s=a.detail.message),n(2,l=!1)}const c=a=>{n(2,l=!0),u.start()};function p(a){G[a?"unshift":"push"](()=>{u=a,n(3,u)})}function f(a){G[a?"unshift":"push"](()=>{u=a,n(3,u)})}return e.$$.update=()=>{e.$$.dirty&2&&console.log(s),e.$$.dirty&1&&console.log(`Mode: ${o}`)},[o,s,l,u,r,i,c,p,f]}class Qt extends X{constructor(t){super(),Q(this,t,Vt,Kt,H,{})}}new Qt({target:document.getElementById("app")});
