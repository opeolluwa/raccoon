import{_ as d,o as m,a as r,b as t,t as f,d as V,B as w,f as B,g as E,r as i,h as v,e as n,F as h,p as g,i as y}from"./index.c229a5ea.js";const I={name:"BaseTextArea",props:{label:{type:String,required:!0},placeholder:{type:String,required:!0},modelValue:{type:String,required:!0}},methods:{}};const S={class:"form__field"},T=["for"],x=["id","placeholder","value"];function C(e,a,o,u,p,c){return m(),r("div",S,[t("label",{for:o.label},f(o.label),9,T),t("textarea",{id:o.label,placeholder:"-- "+o.placeholder+" --",value:o.modelValue,rows:"10",cols:"35"},null,8,x)])}const $=d(I,[["render",C],["__scopeId","data-v-1f63f683"]]),j=V({name:"CreateEmailView",components:{BaseButton:w,BaseTextInput:B,BaseTextarea:$},data:()=>({newEmail:{email:"",recipient:"",subject:"",message:""}}),methods:{async sendEmail(){const e={...this.newEmail};console.log(e),await E.post("http://localhost:3000/emails",e)}}});const U=e=>(g("data-v-068b1eab"),e=e(),y(),e),q=U(()=>t("h2",null,"Compose new email",-1));function k(e,a,o,u,p,c){const s=i("BaseTextInput"),_=i("BaseTextarea"),b=i("BaseButton");return m(),r(h,null,[q,t("div",null,[t("form",{onSubmit:a[4]||(a[4]=v((...l)=>e.sendEmail&&e.sendEmail(...l),["prevent"]))},[t("div",null,[n(s,{label:"recipient email",type:"email",placeholder:"recipient email",modelValue:e.newEmail.email,"onUpdate:modelValue":a[0]||(a[0]=l=>e.newEmail.email=l)},null,8,["modelValue"]),n(s,{placeholder:"recipient name",label:"recipient name",modelValue:e.newEmail.recipient,"onUpdate:modelValue":a[1]||(a[1]=l=>e.newEmail.recipient=l)},null,8,["modelValue"]),n(s,{placeholder:"email subject",label:"email subject",modelValue:e.newEmail.subject,"onUpdate:modelValue":a[2]||(a[2]=l=>e.newEmail.subject=l)},null,8,["modelValue"])]),t("div",null,[n(_,{placeholder:"email body",label:"email body",modelValue:e.newEmail.message,"onUpdate:modelValue":a[3]||(a[3]=l=>e.newEmail.message=l)},null,8,["modelValue"]),n(b,{text:"send message"})])],32)])],64)}const N=d(j,[["render",k],["__scopeId","data-v-068b1eab"]]);export{N as default};
