//! This contains the configuration for the tools which can be used by the agent

use super::identifier::LLMProperties;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolProperties {
    swe_bench_test_endpoint: Option<String>,
    swe_bench_code_editing_llm: Option<LLMProperties>,
    swe_bench_reranking_llm: Option<LLMProperties>,
    swe_bench_long_context_editing_llm: Option<LLMProperties>,
    full_symbol_request: bool,
}

impl ToolProperties {
    pub fn new() -> Self {
        Self {
            swe_bench_test_endpoint: None,
            swe_bench_code_editing_llm: None,
            swe_bench_reranking_llm: None,
            swe_bench_long_context_editing_llm: None,
            full_symbol_request: false,
        }
    }

    pub fn get_full_symbol_request(&self) -> bool {
        self.full_symbol_request
    }

    pub fn set_full_symbol_request(mut self, full_symbol_edit: bool) -> Self {
        self.full_symbol_request = full_symbol_edit;
        self
    }

    pub fn set_long_context_editing_llm(
        mut self,
        swe_bench_long_context_editing_llm: Option<LLMProperties>,
    ) -> Self {
        self.swe_bench_long_context_editing_llm = swe_bench_long_context_editing_llm;
        self
    }

    pub fn get_long_context_editing_llm(&self) -> Option<LLMProperties> {
        self.swe_bench_long_context_editing_llm.clone()
    }

    pub fn set_swe_bench_reranking_llm(
        mut self,
        swe_bench_reranking_llm: Option<LLMProperties>,
    ) -> Self {
        self.swe_bench_reranking_llm = swe_bench_reranking_llm;
        self
    }

    pub fn get_swe_bench_reranking_llm(&self) -> Option<LLMProperties> {
        self.swe_bench_reranking_llm.clone()
    }

    pub fn set_swe_bench_code_editing_llm(
        mut self,
        swe_bench_code_editing_llm: Option<LLMProperties>,
    ) -> Self {
        self.swe_bench_code_editing_llm = swe_bench_code_editing_llm;
        self
    }

    pub fn set_swe_bench_endpoint(mut self, swe_bench_test_endpoint: Option<String>) -> Self {
        self.swe_bench_test_endpoint = swe_bench_test_endpoint;
        self
    }

    pub fn get_swe_bench_test_endpoint(&self) -> Option<String> {
        self.swe_bench_test_endpoint.clone()
    }

    pub fn get_swe_bench_code_editing_llm(&self) -> Option<LLMProperties> {
        self.swe_bench_code_editing_llm.clone()
    }
}
