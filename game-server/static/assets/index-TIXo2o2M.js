var Qe=Object.defineProperty;var Xe=(e,t,n)=>t in e?Qe(e,t,{enumerable:!0,configurable:!0,writable:!0,value:n}):e[t]=n;var ge=(e,t,n)=>(Xe(e,typeof t!="symbol"?t+"":t,n),n);(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const l of document.querySelectorAll('link[rel="modulepreload"]'))r(l);new MutationObserver(l=>{for(const i of l)if(i.type==="childList")for(const o of i.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&r(o)}).observe(document,{childList:!0,subtree:!0});function n(l){const i={};return l.integrity&&(i.integrity=l.integrity),l.referrerPolicy&&(i.referrerPolicy=l.referrerPolicy),l.crossOrigin==="use-credentials"?i.credentials="include":l.crossOrigin==="anonymous"?i.credentials="omit":i.credentials="same-origin",i}function r(l){if(l.ep)return;l.ep=!0;const i=n(l);fetch(l.href,i)}})();function M(){}const Ye=e=>e;function Oe(e,t){for(const n in t)e[n]=t[n];return e}function Le(e){return e()}function Ae(){return Object.create(null)}function H(e){e.forEach(Le)}function Re(e){return typeof e=="function"}function Y(e,t){return e!=e?t==t:e!==t||e&&typeof e=="object"||typeof e=="function"}function Ze(e){return Object.keys(e).length===0}function xe(e,...t){if(e==null){for(const r of t)r(void 0);return M}const n=e.subscribe(...t);return n.unsubscribe?()=>n.unsubscribe():n}function et(e,t,n){e.$$.on_destroy.push(xe(t,n))}function Se(e){return e??""}const Te=typeof window<"u";let tt=Te?()=>window.performance.now():()=>Date.now(),ze=Te?e=>requestAnimationFrame(e):M;const Q=new Set;function qe(e){Q.forEach(t=>{t.c(e)||(Q.delete(t),t.f())}),Q.size!==0&&ze(qe)}function nt(e){let t;return Q.size===0&&ze(qe),{promise:new Promise(n=>{Q.add(t={c:e,f:n})}),abort(){Q.delete(t)}}}function y(e,t){e.appendChild(t)}function D(e,t,n){e.insertBefore(t,n||null)}function P(e){e.parentNode&&e.parentNode.removeChild(e)}function w(e){return document.createElement(e)}function L(e){return document.createElementNS("http://www.w3.org/2000/svg",e)}function R(e){return document.createTextNode(e)}function S(){return R(" ")}function rt(){return R("")}function F(e,t,n,r){return e.addEventListener(t,n,r),()=>e.removeEventListener(t,n,r)}function g(e,t,n){n==null?e.removeAttribute(t):e.getAttribute(t)!==n&&e.setAttribute(t,n)}function ye(e){let t;return{p(...n){t=n,t.forEach(r=>e.push(r))},r(){t.forEach(n=>e.splice(e.indexOf(n),1))}}}function lt(e){return Array.from(e.childNodes)}function it(e,t){t=""+t,e.data!==t&&(e.data=t)}function x(e,t){e.value=t??""}function ot(e,t,{bubbles:n=!1,cancelable:r=!1}={}){return new CustomEvent(e,{detail:t,bubbles:n,cancelable:r})}let te;function ee(e){te=e}function ke(){if(!te)throw new Error("Function called outside component initialization");return te}function st(e){ke().$$.on_mount.push(e)}function ut(e){ke().$$.on_destroy.push(e)}function We(){const e=ke();return(t,n,{cancelable:r=!1}={})=>{const l=e.$$.callbacks[t];if(l){const i=ot(t,n,{cancelable:r});return l.slice().forEach(o=>{o.call(e,i)}),!i.defaultPrevented}return!0}}const V=[],ne=[];let X=[];const De=[],ct=Promise.resolve();let be=!1;function ft(){be||(be=!0,ct.then(Fe))}function ve(e){X.push(e)}const we=new Set;let J=0;function Fe(){if(J!==0)return;const e=te;do{try{for(;J<V.length;){const t=V[J];J++,ee(t),at(t.$$)}}catch(t){throw V.length=0,J=0,t}for(ee(null),V.length=0,J=0;ne.length;)ne.pop()();for(let t=0;t<X.length;t+=1){const n=X[t];we.has(n)||(we.add(n),n())}X.length=0}while(V.length);for(;De.length;)De.pop()();be=!1,we.clear(),ee(e)}function at(e){if(e.fragment!==null){e.update(),H(e.before_update);const t=e.dirty;e.dirty=[-1],e.fragment&&e.fragment.p(e.ctx,t),e.after_update.forEach(ve)}}function dt(e){const t=[],n=[];X.forEach(r=>e.indexOf(r)===-1?t.push(r):n.push(r)),n.forEach(r=>r()),X=t}const ce=new Set;let U;function Ue(){U={r:0,c:[],p:U}}function He(){U.r||H(U.c),U=U.p}function B(e,t){e&&e.i&&(ce.delete(e),e.i(t))}function T(e,t,n,r){if(e&&e.o){if(ce.has(e))return;ce.add(e),U.c.push(()=>{ce.delete(e),r&&(n&&e.d(1),r())}),e.o(t)}else r&&r()}function fe(e){return(e==null?void 0:e.length)!==void 0?e:Array.from(e)}function ht(e,t){e.d(1),t.delete(e.key)}function _t(e,t){T(e,1,1,()=>{t.delete(e.key)})}function Je(e,t,n,r,l,i,o,h,u,s,d,_){let m=e.length,c=i.length,f=m;const p={};for(;f--;)p[e[f].key]=f;const v=[],N=new Map,C=new Map,$=[];for(f=c;f--;){const a=_(l,i,f),b=n(a);let k=o.get(b);k?r&&$.push(()=>k.p(a,t)):(k=s(b,a),k.c()),N.set(b,v[f]=k),b in p&&C.set(b,Math.abs(f-p[b]))}const z=new Set,q=new Set;function G(a){B(a,1),a.m(h,d),o.set(a.key,a),d=a.first,c--}for(;m&&c;){const a=v[c-1],b=e[m-1],k=a.key,O=b.key;a===b?(d=a.first,m--,c--):N.has(O)?!o.has(k)||z.has(k)?G(a):q.has(O)?m--:C.get(k)>C.get(O)?(q.add(k),G(a)):(z.add(O),m--):(u(b,o),m--)}for(;m--;){const a=e[m];N.has(a.key)||u(a,o)}for(;c;)G(v[c-1]);return H($),v}function ae(e){e&&e.c()}function re(e,t,n){const{fragment:r,after_update:l}=e.$$;r&&r.m(t,n),ve(()=>{const i=e.$$.on_mount.map(Le).filter(Re);e.$$.on_destroy?e.$$.on_destroy.push(...i):H(i),e.$$.on_mount=[]}),l.forEach(ve)}function le(e,t){const n=e.$$;n.fragment!==null&&(dt(n.after_update),H(n.on_destroy),n.fragment&&n.fragment.d(t),n.on_destroy=n.fragment=null,n.ctx=[])}function pt(e,t){e.$$.dirty[0]===-1&&(V.push(e),ft(),e.$$.dirty.fill(0)),e.$$.dirty[t/31|0]|=1<<t%31}function ie(e,t,n,r,l,i,o=null,h=[-1]){const u=te;ee(e);const s=e.$$={fragment:null,ctx:[],props:i,update:M,not_equal:l,bound:Ae(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(t.context||(u?u.$$.context:[])),callbacks:Ae(),dirty:h,skip_bound:!1,root:t.target||u.$$.root};o&&o(s.root);let d=!1;if(s.ctx=n?n(e,t.props||{},(_,m,...c)=>{const f=c.length?c[0]:m;return s.ctx&&l(s.ctx[_],s.ctx[_]=f)&&(!s.skip_bound&&s.bound[_]&&s.bound[_](f),d&&pt(e,_)),m}):[],s.update(),d=!0,H(s.before_update),s.fragment=r?r(s.ctx):!1,t.target){if(t.hydrate){const _=lt(t.target);s.fragment&&s.fragment.l(_),_.forEach(P)}else s.fragment&&s.fragment.c();t.intro&&B(e.$$.fragment),re(e,t.target,t.anchor),Fe()}ee(u)}class oe{constructor(){ge(this,"$$");ge(this,"$$set")}$destroy(){le(this,1),this.$destroy=M}$on(t,n){if(!Re(n))return M;const r=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return r.push(n),()=>{const l=r.indexOf(n);l!==-1&&r.splice(l,1)}}$set(t){this.$$set&&!Ze(t)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}const mt="4";typeof window<"u"&&(window.__svelte||(window.__svelte={v:new Set})).v.add(mt);function gt(e){let t=yt(e);return t!==0||(t=wt(e),t!==0)||(t=bt(e),t!==0)||(t=vt(e)),t}function yt(e){for(let t=0;t<3;t++)e:for(let n=0;n<7;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t+l][n]!==r)continue e;return r}}return 0}function wt(e){for(let t=0;t<6;t++)e:for(let n=0;n<4;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t][n+l]!==r)continue e;return r}}return 0}function bt(e){for(let t=0;t<3;t++)e:for(let n=0;n<4;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t+l][n+l]!==r)continue e;return r}}return 0}function vt(e){for(let t=3;t<6;t++)e:for(let n=0;n<4;n++){let r=e[t][n];if(r!==0){for(let l=1;l<4;l++)if(e[t-l][n+l]!==r)continue e;return r}}return 0}const K=[];function Ke(e,t=M){let n;const r=new Set;function l(h){if(Y(e,h)&&(e=h,n)){const u=!K.length;for(const s of r)s[1](),K.push(s,e);if(u){for(let s=0;s<K.length;s+=2)K[s][0](K[s+1]);K.length=0}}}function i(h){l(h(e))}function o(h,u=M){const s=[h,u];return r.add(s),r.size===1&&(n=t(l,i)||M),h(e),()=>{r.delete(s),r.size===0&&n&&(n(),n=null)}}return{set:l,update:i,subscribe:o}}const Ve=Ke({firstPlayerIsRed:!0,isOnePlayer:!0,aiDifficulty:"easy"});function Ce(e){return Object.prototype.toString.call(e)==="[object Date]"}function $t(e){const t=e-1;return t*t*t+1}function $e(e,t){if(e===t||e!==e)return()=>e;const n=typeof e;if(n!==typeof t||Array.isArray(e)!==Array.isArray(t))throw new Error("Cannot interpolate values of different type");if(Array.isArray(e)){const r=t.map((l,i)=>$e(e[i],l));return l=>r.map(i=>i(l))}if(n==="object"){if(!e||!t)throw new Error("Object cannot be null");if(Ce(e)&&Ce(t)){e=e.getTime(),t=t.getTime();const i=t-e;return o=>new Date(e+o*i)}const r=Object.keys(t),l={};return r.forEach(i=>{l[i]=$e(e[i],t[i])}),i=>{const o={};return r.forEach(h=>{o[h]=l[h](i)}),o}}if(n==="number"){const r=t-e;return l=>e+l*r}throw new Error(`Cannot interpolate ${n} values`)}function kt(e,t={}){const n=Ke(e);let r,l=e;function i(o,h){if(e==null)return n.set(e=o),Promise.resolve();l=o;let u=r,s=!1,{delay:d=0,duration:_=400,easing:m=Ye,interpolate:c=$e}=Oe(Oe({},t),h);if(_===0)return u&&(u.abort(),u=null),n.set(e=l),Promise.resolve();const f=tt()+d;let p;return r=nt(v=>{if(v<f)return!0;s||(p=c(e,o),typeof _=="function"&&(_=_(e,o)),s=!0),u&&(u.abort(),u=null);const N=v-f;return N>_?(n.set(e=o),!1):(n.set(e=p(m(N/_))),!0)}),r.promise}return{set:i,update:(o,h)=>i(o(l,e),h),subscribe:n.subscribe}}function Et(e){let t,n;return{c(){t=L("circle"),g(t,"cx",e[2]),g(t,"cy",e[1]),g(t,"r","45"),g(t,"fill",n=e[0]==="red"?"#E53734":"#04BEBE")},m(r,l){D(r,t,l)},p(r,[l]){l&2&&g(t,"cy",r[1]),l&1&&n!==(n=r[0]==="red"?"#E53734":"#04BEBE")&&g(t,"fill",n)},i:M,o:M,d(r){r&&P(t)}}}function Pt(e,t,n){let r,{row:l}=t,{col:i}=t,{color:o}=t,h=50+100*i,u=kt(50,{delay:50,easing:$t,duration:(s,d)=>Math.sqrt(Math.abs(s-d)*400)});return et(e,u,s=>n(1,r=s)),u.set(150+100*(5-l)),e.$$set=s=>{"row"in s&&n(4,l=s.row),"col"in s&&n(5,i=s.col),"color"in s&&n(0,o=s.color)},[o,r,h,u,l,i]}class Ot extends oe{constructor(t){super(),ie(this,t,Pt,Et,Y,{row:4,col:5,color:0})}}function Me(e,t,n){const r=e.slice();return r[3]=t[n].row,r[4]=t[n].col,r[5]=t[n].color,r[7]=n,r}function Ie(e,t){let n,r,l;return r=new Ot({props:{row:t[3],col:t[4],color:t[5]}}),{key:e,first:null,c(){n=rt(),ae(r.$$.fragment),this.first=n},m(i,o){D(i,n,o),re(r,i,o),l=!0},p(i,o){t=i;const h={};o&1&&(h.row=t[3]),o&1&&(h.col=t[4]),o&1&&(h.color=t[5]),r.$set(h)},i(i){l||(B(r.$$.fragment,i),l=!0)},o(i){T(r.$$.fragment,i),l=!1},d(i){i&&P(n),le(r,i)}}}function At(e){let t,n,r,l,i,o,h,u=[],s=new Map,d,_,m=fe(e[0]);const c=f=>f[7];for(let f=0;f<m.length;f+=1){let p=Me(e,m,f),v=c(p);s.set(v,u[f]=Ie(v,p))}return{c(){t=L("svg"),n=L("defs"),r=L("pattern"),l=L("circle"),i=L("mask"),o=L("rect"),h=L("rect");for(let f=0;f<u.length;f+=1)u[f].c();d=L("rect"),g(l,"cx","50"),g(l,"cy","50"),g(l,"r","45"),g(l,"fill","black"),g(r,"id","cell-pattern"),g(r,"patternUnits","userSpaceOnUse"),g(r,"width","100"),g(r,"height","100"),g(o,"width","700"),g(o,"height","700"),g(o,"fill","white"),g(h,"width","700"),g(h,"height","700"),g(h,"fill","url(#cell-pattern)"),g(i,"id","cell-mask"),g(d,"x","0"),g(d,"y","100"),g(d,"height","600"),g(d,"width","700"),g(d,"fill","#303030"),g(d,"mask","url(#cell-mask)"),g(t,"width","350px"),g(t,"viewBox","0 0 700 700"),g(t,"xmlns","http://www.w3.org/2000/svg"),g(t,"class","svelte-16b1qih")},m(f,p){D(f,t,p),y(t,n),y(n,r),y(r,l),y(n,i),y(i,o),y(i,h);for(let v=0;v<u.length;v+=1)u[v]&&u[v].m(t,null);y(t,d),_=!0},p(f,[p]){p&1&&(m=fe(f[0]),Ue(),u=Je(u,p,c,1,f,m,s,t,_t,Ie,d,Me),He())},i(f){if(!_){for(let p=0;p<m.length;p+=1)B(u[p]);_=!0}},o(f){for(let p=0;p<u.length;p+=1)T(u[p]);_=!1},d(f){f&&P(t);for(let p=0;p<u.length;p+=1)u[p].d()}}}function St(e,t,n){let r=[];function l(o,h,u){n(0,r=[...r,{row:o,col:h,color:u}])}function i(){n(0,r=[])}return[r,l,i]}class Dt extends oe{constructor(t){super(),ie(this,t,St,At,Y,{placePiece:1,clear:2})}get placePiece(){return this.$$.ctx[1]}get clear(){return this.$$.ctx[2]}}function je(e,t,n){const r=e.slice();return r[20]=t[n],r[21]=t,r[22]=n,r}function Ne(e,t){let n,r=t[22],l,i;const o=()=>t[7](n,r),h=()=>t[7](null,r);function u(){return t[8](t[22])}return{key:e,first:null,c(){n=w("button"),g(n,"class","svelte-ufvfu4"),this.first=n},m(s,d){D(s,n,d),o(),l||(i=F(n,"click",u),l=!0)},p(s,d){t=s,r!==t[22]&&(h(),r=t[22],o())},d(s){s&&P(n),h(),l=!1,i()}}}function Ct(e){let t,n,r,l,i,o=[],h=new Map,u,s,d={};r=new Dt({props:d}),e[6](r);let _=fe(Array(7));const m=c=>c[22];for(let c=0;c<_.length;c+=1){let f=je(e,_,c),p=m(f);h.set(p,o[c]=Ne(p,f))}return{c(){t=w("div"),n=w("div"),ae(r.$$.fragment),l=S(),i=w("div");for(let c=0;c<o.length;c+=1)o[c].c();g(n,"class","svelte-ufvfu4"),g(i,"class","svelte-ufvfu4"),g(t,"class",u=Se(e[0]?"":"dark")+" svelte-ufvfu4")},m(c,f){D(c,t,f),y(t,n),re(r,n,null),y(t,l),y(t,i);for(let p=0;p<o.length;p+=1)o[p]&&o[p].m(i,null);s=!0},p(c,[f]){const p={};r.$set(p),f&10&&(_=fe(Array(7)),o=Je(o,f,m,1,c,_,h,i,ht,Ne,null,je)),(!s||f&1&&u!==(u=Se(c[0]?"":"dark")+" svelte-ufvfu4"))&&g(t,"class",u)},i(c){s||(B(r.$$.fragment,c),s=!0)},o(c){T(r.$$.fragment,c),s=!1},d(c){c&&P(t),e[6](null),le(r);for(let f=0;f<o.length;f+=1)o[f].d()}}}function Mt(e,t,n){const r=We();let l=!1,i=!0,o=!0,h="easy";const u=Ve.subscribe(a=>{i=a.firstPlayerIsRed,o=a.isOnePlayer,h=a.aiDifficulty});ut(()=>{u()}),st(()=>{p()});let s=0,d=[],_=Array(7).fill(!1),m=Array.from({length:6},()=>Array(7).fill(0)),c;function f(){_=Array(7).fill(!1),m=Array.from({length:6},()=>Array(7).fill(0)),s=0,n(0,l=!0),d.forEach(a=>a.disabled=!1),c.clear()}function p(){n(0,l=!1),d.forEach(a=>a.disabled=!0)}function v(a){r("gameover",{winner:a})}function N(a){for(let b=0;b<m.length;b++)if(m[b][a]===0)return b;return 6}function C(a){d.forEach(I=>{I.disabled=!0});let b=N(a);b===5&&(_[a]=!0);let k=s%2===0?1:-1;m[b][a]=k,c.placePiece(b,a,k===1?"red":"blue"),s++;let O=gt(m);if(O!==0){v(O==1?"Red":"Blue");return}if(s==42&&v("Draw"),!o||!o&&(i&&k===1||!i&&k===-1))for(let I=0;I<_.length;I++)n(1,d[I].disabled=_[I],d);else $()}async function $(){let a=Date.now(),W=(await(await fetch("/ai",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({board:m,difficulty:h})})).json()).move,I=Date.now()-a;I<2e3?setTimeout(()=>{console.log("here"),C(W)},I):(console.log("here"),C(W))}function z(a){ne[a?"unshift":"push"](()=>{c=a,n(2,c)})}function q(a,b){ne[a?"unshift":"push"](()=>{d[b]=a,n(1,d)})}return[l,d,c,C,f,p,z,q,a=>C(a)]}class It extends oe{constructor(t){super(),ie(this,t,Mt,Ct,Y,{startGame:4,endGame:5})}get startGame(){return this.$$.ctx[4]}get endGame(){return this.$$.ctx[5]}}function jt(e){let t,n,r,l,i,o,h,u,s,d,_,m,c,f,p,v,N,C,$,z,q,G,a,b,k,O,W,I,se,E,ue,Ee,de,Z,he,_e,pe,me,Pe;return he=ye(e[5][0]),_e=ye(e[5][1]),pe=ye(e[5][2]),{c(){t=w("h2"),t.textContent="Game Options",n=S(),r=w("div"),l=w("div"),i=w("h3"),i.textContent="Player 1 Color",o=S(),h=w("label"),u=w("input"),s=R(`
      Red`),d=S(),_=w("label"),m=w("input"),c=R(`
      Blue`),f=S(),p=w("div"),v=w("h3"),v.textContent="Number Of Players",N=S(),C=w("label"),$=w("input"),z=R(`
      One Player`),q=S(),G=w("label"),a=w("input"),b=R(`
      Two Player`),k=S(),O=w("div"),W=w("h3"),W.textContent="AI Difficulty",I=S(),se=w("label"),E=w("input"),Ee=R(`
      Easy`),de=S(),Z=w("button"),Z.textContent="Start",u.checked=!0,g(u,"name","color"),g(u,"type","radio"),u.__value=!0,x(u,u.__value),g(m,"name","color"),g(m,"type","radio"),m.__value=!1,x(m,m.__value),$.checked=!0,g($,"name","playerNum"),g($,"type","radio"),$.__value=!0,x($,$.__value),g(a,"name","playerNum"),g(a,"type","radio"),a.__value=!1,x(a,a.__value),E.checked=!0,g(E,"name","AiDifficulty"),g(E,"type","radio"),E.__value="easy",x(E,E.__value),E.disabled=ue=!e[0],g(r,"class","container svelte-hvlkr9"),he.p(E),_e.p($,a),pe.p(u,m)},m(A,j){D(A,t,j),D(A,n,j),D(A,r,j),y(r,l),y(l,i),y(l,o),y(l,h),y(h,u),u.checked=u.__value===e[1],y(h,s),y(l,d),y(l,_),y(_,m),m.checked=m.__value===e[1],y(_,c),y(r,f),y(r,p),y(p,v),y(p,N),y(p,C),y(C,$),$.checked=$.__value===e[0],y(C,z),y(p,q),y(p,G),y(G,a),a.checked=a.__value===e[0],y(G,b),y(r,k),y(r,O),y(O,W),y(O,I),y(O,se),y(se,E),E.checked=E.__value===e[2],y(se,Ee),D(A,de,j),D(A,Z,j),me||(Pe=[F(u,"change",e[4]),F(m,"change",e[6]),F($,"change",e[7]),F(a,"change",e[8]),F(E,"change",e[9]),F(Z,"click",e[10])],me=!0)},p(A,[j]){j&2&&(u.checked=u.__value===A[1]),j&2&&(m.checked=m.__value===A[1]),j&1&&($.checked=$.__value===A[0]),j&1&&(a.checked=a.__value===A[0]),j&1&&ue!==(ue=!A[0])&&(E.disabled=ue),j&4&&(E.checked=E.__value===A[2])},i:M,o:M,d(A){A&&(P(t),P(n),P(r),P(de),P(Z)),he.r(),_e.r(),pe.r(),me=!1,H(Pe)}}}function Nt(e,t,n){let r=We(),l=!0,i=!0,o=l?"easy":"n/a";const h=[[],[],[]];function u(){i=this.__value,n(1,i)}function s(){i=this.__value,n(1,i)}function d(){l=this.__value,n(0,l)}function _(){l=this.__value,n(0,l)}function m(){o=this.__value,n(2,o)}const c=f=>{r("gameStart",{})};return e.$$.update=()=>{e.$$.dirty&7&&Ve.set({firstPlayerIsRed:i,aiDifficulty:o,isOnePlayer:l})},[l,i,o,r,u,h,s,d,_,m,c]}class Bt extends oe{constructor(t){super(),ie(this,t,Nt,jt,Y,{})}}function Be(e){let t,n;return t=new Bt({}),t.$on("gameStart",e[3]),{c(){ae(t.$$.fragment)},m(r,l){re(t,r,l),n=!0},p:M,i(r){n||(B(t.$$.fragment,r),n=!0)},o(r){T(t.$$.fragment,r),n=!1},d(r){le(t,r)}}}function Ge(e){let t,n=e[2]==="Draw"?"Draw":e[2]+" Wins",r;return{c(){t=w("h3"),r=R(n)},m(l,i){D(l,t,i),y(t,r)},p(l,i){i&4&&n!==(n=l[2]==="Draw"?"Draw":l[2]+" Wins")&&it(r,n)},d(l){l&&P(t)}}}function Gt(e){let t,n,r,l,i,o,h,u,s,d=!e[0]&&Be(e),_=e[2]!==""&&Ge(e),m={};return u=new It({props:m}),e[4](u),u.$on("gameover",e[5]),{c(){t=w("head"),t.innerHTML="<title>Four In a Row</title>",n=S(),r=w("main"),l=w("h1"),l.textContent="Four In a Row",i=S(),d&&d.c(),o=S(),_&&_.c(),h=S(),ae(u.$$.fragment),g(r,"class","svelte-be9z7h")},m(c,f){D(c,t,f),D(c,n,f),D(c,r,f),y(r,l),y(r,i),d&&d.m(r,null),y(r,o),_&&_.m(r,null),y(r,h),re(u,r,null),s=!0},p(c,[f]){c[0]?d&&(Ue(),T(d,1,1,()=>{d=null}),He()):d?(d.p(c,f),f&1&&B(d,1)):(d=Be(c),d.c(),B(d,1),d.m(r,o)),c[2]!==""?_?_.p(c,f):(_=Ge(c),_.c(),_.m(r,h)):_&&(_.d(1),_=null);const p={};u.$set(p)},i(c){s||(B(d),B(u.$$.fragment,c),s=!0)},o(c){T(d),T(u.$$.fragment,c),s=!1},d(c){c&&(P(t),P(n),P(r)),d&&d.d(),_&&_.d(),e[4](null),le(u)}}}function Lt(e,t,n){let r=!1,l,i="";const o=s=>{n(0,r=!0),l.startGame()};function h(s){ne[s?"unshift":"push"](()=>{l=s,n(1,l)})}return[r,l,i,o,h,s=>{l.endGame(),n(0,r=!1),n(2,i=s.detail.winner)}]}class Rt extends oe{constructor(t){super(),ie(this,t,Lt,Gt,Y,{})}}new Rt({target:document.getElementById("app")});