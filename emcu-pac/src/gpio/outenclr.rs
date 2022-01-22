#[doc = "Register `OUTENCLR` reader"]
pub struct R(crate::R<OUTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTENCLR` writer"]
pub struct W(crate::W<OUTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OUTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Enable Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outenclr](index.html) module"]
pub struct OUTENCLR_SPEC;
impl crate::RegisterSpec for OUTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outenclr::R](R) reader structure"]
impl crate::Readable for OUTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outenclr::W](W) writer structure"]
impl crate::Writable for OUTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTENCLR to value 0"]
impl crate::Resettable for OUTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
