use crate::{
    application::{student_information::StudentInformation, USaintClient},
    define_elements,
    webdynpro::{
        command::element::layout::TabStripTabSelectCommand,
        element::{
            action::Button, definition::ElementDefinition, layout::tab_strip::item::TabStripItem,
            selection::ComboBox, text::InputField,
        },
        error::WebDynproError,
    },
};

#[derive(Clone, Debug)]
/// 연구비 입급 계좌 정보
pub struct StudentResearchBankAccountInformation {
    bank: Option<String>,
    account_number: Option<String>,
    holder: Option<String>,
}

impl<'a> StudentResearchBankAccountInformation {
    // 연구비 입금 계좌
    define_elements! {
        // 연구비 입금 계좌 탭
        TAB_RES_ACCOUNT: TabStripItem<'a> = "ZCMW1001.ID_0001:VIW_MAIN.TAB_RES_ACCOUNT";
        // 은행구분
        BANK_TEXT: ComboBox<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.BANK_TEXT";
        // 은행계좌번호
        BANKN_TEXT: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.BANKN_TEXT";
        // 예금주
        ZKOINH_TEXT: InputField<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.ZKOINH_TEXT";
        #[allow(unused)]
        MODIFY_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.MODIFY_BUTTON";
        #[allow(unused)]
        SAVE_BUTTON: Button<'a> = "ZCMW1001.ID_0001:VIW_TAB_RES_ACCOUNT.SAVE_BUTTON";
    }

    pub(crate) async fn with_client(client: &mut USaintClient) -> Result<Self, WebDynproError> {
        client
            .send(TabStripTabSelectCommand::new(
                StudentInformation::TAB_ADDITION,
                Self::TAB_RES_ACCOUNT,
                6,
                0,
            ))
            .await?;
        Ok(Self {
            bank: Self::BANK_TEXT
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            account_number: Self::BANKN_TEXT
                .from_body(client.body())?
                .value()
                .map(str::to_string),
            holder: Self::ZKOINH_TEXT
                .from_body(client.body())?
                .value()
                .map(str::to_string),
        })
    }
    
    /// 학생 연구비 입금 계좌의 은행을 반환합니다.
    pub fn bank(&self) -> Option<&str> {
        self.bank.as_ref().map(String::as_str)
    }
    
    /// 학생 연구비 입금 계좌번호를 반환합니다.
    pub fn account_number(&self) -> Option<&str> {
        self.account_number.as_ref().map(String::as_str)
    }
    
    /// 학생 연구비 입금 계좌의 예금주를 반환합니다.
    pub fn holder(&self) -> Option<&str> {
        self.holder.as_ref().map(String::as_str)
    }
}