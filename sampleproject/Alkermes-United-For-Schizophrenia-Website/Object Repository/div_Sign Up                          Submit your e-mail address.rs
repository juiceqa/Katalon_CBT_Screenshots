<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Sign Up                          Submit your e-mail address</name>
   <tag></tag>
   <elementGuidId>93cb7d63-9ac1-458e-98ff-c57d83cfdf04</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='signup-modal-form-group']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>CSS</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>signup-modal-form-group</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>modal-group</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
          
            Sign Up
            
              Submit your e-mail address to receive notifications about updates to this website, announcements, and information from Alkermes and United for Schizophrenia.
            
          

          
            
            .mktoForm .mktoButtonWrap.mktoInset .mktoButton {
color:#000;
background:#fff;
border:1px solid #aeb0b6;
padding:0.4em 1em;
font-size:1em;
box-shadow: 1px 1px 6px 1px #ccc;
background-color:#f5f5f5;
background-image: -webkit-gradient(linear, left top, left bottom, from(#f5f5f5), to(#dfdfdf));
background-image: -webkit-linear-gradient(top, #f5f5f5, #dfdfdf);
background-image: -moz-linear-gradient(top, #f5f5f5, #dfdfdf);
background-image: linear-gradient(to bottom, #f5f5f5, #dfdfdf);
}
.mktoForm .mktoButtonWrap.mktoInset .mktoButton:hover {
border:1px solid #999;
}
.mktoForm .mktoButtonWrap.mktoInset .mktoButton:focus {
outline:none;
border:1px solid #999;
}
.mktoForm .mktoButtonWrap.mktoInset .mktoButton:active{
box-shadow:inset 1px 1px 6px 1px #ccc;
background-color:#dfdfdf;
background-image: -webkit-gradient(linear, left top, left bottom, from(#dfdfdf), to(#f5f5f5));
background-image: -webkit-linear-gradient(top, #dfdfdf, #f5f5f5);
background-image: -moz-linear-gradient(top, #dfdfdf, #f5f5f5);
background-image: linear-gradient(to bottom, #dfdfdf, #f5f5f5);
}
*First Name*Last Name*E-mail Address*Zip Code*SpecialtySelect...PsychiatristPharmacistNurse PractitionerPhysician AssistantPsychiatric NurseCase ManagerSocial WorkerOther*I understand and agree to the Alkermes Privacy Policy and Terms of Use.Sign Up
            MktoForms2.loadForm(&quot;//app-ab26.marketo.com&quot;, '663-TGT-323', 1167);
            
              MktoForms2.whenReady(function (form) {
                console.log(&quot;EC:0&quot;);
                var formId = form.getId();
                var formElement = form.getFormElem()[0];
                var submitBtn = document.querySelector('#mktoForm_' + formId + ' > div.mktoButtonRow > span > button');

                // Scopes actions to a specific form
                if (formId === 1167) {
                  // Set GTM tags for submit button
                  $(submitBtn).attr({
                    'data-gtm-event-tracker': 'event',
                    'data-gtm-event-category': 'overlay',
                    'data-gtm-event-action' : 'click',
                    'data-gtm-event-label': 'sign up'
                  });

                  form.onValidate(function (valid) {
                    highlightInvalidMarketoFields(form);

                    // if (form.submittable()) {
                    //   console.log(&quot;EC:5&quot;);
                    //   $(formElement).removeClass('invalid');
                    // } else {
                    //   console.log(&quot;EC:6&quot;);
                    // }
                  });

                  form.onSuccess(function (values, followUpUrl) {
                    console.log(&quot;EC:5&quot;);
                    $('#signup-modal-form-group').hide();
                    $('#signup-modal-success-group').show();

                    // reset submit btn
                    $(submitBtn).removeAttr('disabled');
                    $(submitBtn).html('Sign Up');

                    formElement.reset();

                    return false;
                  });
                }
              });
            
          
        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;signup-modal-form-group&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='signup-modal-form-group']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='signup-modal']/div/div/div[2]/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Am J Psychiatry'])[1]/following::div[6]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='References:'])[1]/following::div[6]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div/div[2]/div</value>
   </webElementXpaths>
</WebElementEntity>
